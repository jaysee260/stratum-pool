# Captured miner traffic

What real hardware sends when it connects. Each capture is the pool's own log:
lines starting with `<-` are exactly what the miner sent.

The payout address in the worker name is replaced with `<address>`.

## Bitaxe (BM1370, firmware v2.15.4), Nov 25, 2026

```
connection from 192.168.1.x:58104
<- {"id":1,"method":"mining.configure","params":[["version-rolling"],{"version-rolling.mask":"ffffffff"}]}
not a message: unsupported method: mining.configure
<- {"id":2,"method":"mining.subscribe","params":["bitaxe/BM1370/v2.15.4"]}
received message: Subscribe(Subscribe { user_agent: Some("bitaxe/BM1370/v2.15.4") })
<- {"id":3,"method":"mining.authorize","params":["<address>.humble_gamma1","x"]}
received message: Authorize(Authorize { worker_name: "<address>.humble_gamma1", password: Some("x") })
<- {"id":4,"method":"mining.suggest_difficulty","params":[1000]}
not a message: unsupported method: mining.suggest_difficulty
```

After line 4 the miner goes quiet. It is waiting for `mining.notify`, the first
job, which the pool cannot send yet.

### Line by line

| Id | Method | The miner is saying | The pool today |
| --- | --- | --- | --- |
| 1 | `mining.configure` | "I can vary the header's version bits. Which bits may I use?" | Rejects it, sends no reply |
| 2 | `mining.subscribe` | "I'm here and I want work." Names its firmware and chip. | Replies with `extranonce1` and the `extranonce2` size |
| 3 | `mining.authorize` | "This is who I am." | Replies `true`, then sends `mining.set_difficulty` |
| 4 | `mining.suggest_difficulty` | "I'd prefer a difficulty of about 1000." | Rejects it, sends no reply |

### What it shows

- **The miner does not wait for replies.** It sent `mining.subscribe` without
  an answer to `mining.configure`. Replies are matched by `id`, not by order.
- **Params are not always text.** `mining.configure` sends a list and an
  object; `mining.suggest_difficulty` sends a number. The loose `Request`
  (`params` as a list of any JSON value) accepts both, so they are rejected
  cleanly as unknown methods and not as malformed lines.
- **The worker name is `address.nickname`.** The part before the dot is a
  Bitcoin address. Solo pools use it to decide where to pay.
- **Difficulty 1 is far too low for this hardware.** The miner asks for 1000.
- **A request the pool does not support gets no reply.** A miner may wait on
  it. Every request with an id should get an answer, even if it is an error.

### The two new messages

**`mining.configure`** negotiates extensions. Here the miner asks for version
rolling: permission to change some bits of the block header's version field as
one more thing to vary, besides the nonce and `extranonce2`. The mask
`ffffffff` means it can change any of the 32 bits. The pool answers with the
bits it allows, normally `1fffe000`:

```
{"id":1,"result":{"version-rolling":true,"version-rolling.mask":"1fffe000"},"error":null}
```

A miner that uses version rolling adds a sixth param to `mining.submit`: the
version bits it chose.

**`mining.suggest_difficulty`** is a request, not a command. The pool may
honour it by sending `mining.set_difficulty` with that value, or ignore it.
