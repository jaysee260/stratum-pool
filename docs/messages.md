# Stratum messages, in plain terms

What each Stratum V1 message means, what it looks like on the wire, and
whether this pool handles it yet.

Picture the pool as a foreman handing out lottery tickets to scratch, and the
miner as a worker who scratches them very fast. Almost every ticket loses. The
messages below are how the two of them agree on which tickets to scratch and
how the worker proves it is working.

Every message is one line of JSON, ended by a newline, over a TCP connection
that stays open.

| Message | Direction | Meaning | Status |
| --- | --- | --- | --- |
| `mining.subscribe` | Miner to pool | "I'm here and I want work" | Handled; each connection gets its own `extranonce1` |
| `mining.authorize` | Miner to pool | "This is who I am" | Handled, always answers `true` |
| `mining.submit` | Miner to pool | "I found a good one" | Parsed, always answers `true`; not checked yet |
| `mining.set_difficulty` | Pool to miner | "Only show me tickets this good" | Sent after authorize, fixed at 1 |
| `mining.notify` | Pool to miner | "Here's the job" | Not sent yet |

## Miner to pool

These are requests. Each carries an `id`, and the pool's reply echoes the same
`id` so the miner can match the answer to its question.

### `mining.subscribe`: "I'm here and I want work"

The first thing a miner says. It may name its software.

```
{"id": 1, "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}
```

The pool answers with three things:

```
{"id":1,"result":[[["mining.set_difficulty","1"],["mining.notify","1"]],"00000001",8],"error":null}
```

1. **Subscriptions.** A list of pairs that miners mostly ignore. The `"1"`s are
   subscription ids.
2. **`extranonce1`** (`"00000001"`). A tag unique to this connection, written
   as hex. It guarantees that two miners never scratch the same tickets, so no
   work is duplicated.
3. **`extranonce2` size** (`8`). How many bytes the miner may fill in itself.
   This is the miner's own space to vary, on top of the nonce.

### `mining.authorize`: "This is who I am"

The miner gives a worker name and a password. Most pools ignore the password.

```
{"id": 2, "method": "mining.authorize", "params": ["jc.worker1", "x"]}
```

The pool uses the name to know who gets credit for the work. It answers `true`
or `false`:

```
{"id":2,"result":true,"error":null}
```

### `mining.submit`: "I found a good one"

Sent each time the miner finds a result that clears the pool's bar.

```
{"id": 4, "method": "mining.submit", "params": ["jc.worker1", "job1", "0000000000000001", "6740f1a2", "1a2b3c4d"]}
```

The five params are the pieces only the miner knows:

| Param | Meaning |
| --- | --- |
| Worker name | Who gets the credit |
| Job id | Which job this answers |
| `extranonce2` | The bytes the miner chose for its own space |
| `ntime` | The time the miner put in the block header |
| `nonce` | The number that made the hash come out low |

The pool rebuilds the block header from these and what it already knows about
the job, hashes it, and checks that the hash is low enough. `true` means the
share was accepted and credited:

```
{"id":4,"result":true,"error":null}
```

A rejected share gets a null result and an error of the form
`[code, message, null]`. Common reasons are a stale job, a duplicate share, and
a hash that does not meet the difficulty.

## Pool to miner

These are notifications. Their `id` is null, because the pool is not answering
a question and expects no reply.

### `mining.set_difficulty`: "Only show me tickets at least this good"

```
{"id":null,"method":"mining.set_difficulty","params":[512]}
```

A real block is nearly impossible to find, so the pool sets an easier bar. Each
result that clears it is a **share**: proof that the miner is working. A higher
difficulty means fewer, more valuable shares. The pool tunes it per miner so
each one reports every few seconds rather than flooding it.

### `mining.notify`: "Here's the job"

Carries everything the miner needs to build a block header.

| Param | Meaning |
| --- | --- |
| Job id | The name the miner quotes back in `mining.submit` |
| Previous block hash | The block this one builds on |
| Coinbase part 1 | The first half of the coinbase transaction |
| Coinbase part 2 | The second half |
| Merkle branches | The hashes needed to compute the merkle root |
| Version | The block version |
| `nbits` | The network's difficulty, in compact form |
| `ntime` | The current time |
| `clean_jobs` | True means drop all earlier work |

The miner puts `extranonce1` and its own `extranonce2` between the two coinbase
halves. That is what makes each miner's work different from every other
miner's.

`clean_jobs` is true when someone has just found a block. Work on the old job
can no longer win, so the miner must start on this one at once.

## The whole conversation

```
miner: subscribe          ->  pool: here's your extranonce1
miner: authorize          ->  pool: true
                              pool: set_difficulty
                              pool: notify (job 1)
miner: submit             ->  pool: true
miner: submit             ->  pool: true
                              pool: notify (job 2, a new block arrived)
...and so on, for days
```

The pool handles the first three lines today. See [stages.md](stages.md) for
what comes next.

## Not covered here

Real hardware also sends `mining.configure` (to negotiate version rolling),
`mining.suggest_difficulty` and sometimes `mining.extranonce.subscribe`. The
pool rejects all three as unsupported methods for now. A real session with the
first two is in [captures.md](captures.md).
