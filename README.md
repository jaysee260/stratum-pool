# stratum-pool

A Bitcoin mining pool written from scratch in Rust, speaking Stratum V1. This
is a learning project: every piece is built by hand to understand how a pool
works, from parsing a miner's first line to submitting a block.

It is not ready for real mining. Do not point hardware at it on mainnet.

## Status

The pool can read and parse what a miner sends, and answers each message
according to its type. The values in the replies are placeholders: a fixed
`extranonce1`, and `true` for every authorize and submit.

| Stage | What | Status |
| --- | --- | --- |
| 1 | Project setup | Done |
| 2 | Stratum message codec | Done |
| 3 | TCP listener | In progress |
| 4 | Share validation | Not started |
| 5 | Bitcoin Core integration | Not started |

Each stage's checklist and the later stages are in
[docs/stages.md](docs/stages.md).

## Try it

Run the tests:

```
cargo test
```

Run the listener, which accepts one connection on port 3333, prints each
message it receives and replies to it:

```
cargo run
```

In a second terminal, connect and paste a line, then press Enter:

```
nc 127.0.0.1 3333
{"id": 1, "method": "mining.subscribe", "params": ["bitaxe/BM1366/v2.4.0"]}
```

The first terminal prints the parsed message:

```
received message: Subscribe(Subscribe { user_agent: Some("bitaxe/BM1366/v2.4.0") })
```

The second terminal receives the reply:

```
{"id":1,"result":[[["mining.set_difficulty","1"],["mining.notify","1"]],"00000001",8],"error":null}
```

A line that is not valid, such as `GET / HTTP/1.1`, prints an error and the
listener carries on. It gets no reply yet.

## Layout

```
src/main.rs            TCP listener: reads lines, parses them, logs and replies
src/stratum.rs         Stratum messages: types, parsers, serialization
src/stratum/tests.rs   Tests for the codec
docs/stages.md         Build stages and their status
```

## How a line is handled

1. `serde_json` reads the line into a loose `Request` (`id`, `method`,
   `params`). Anything that is not a JSON-RPC request is rejected here.
2. `parse_message` reads `method` and picks the parser for it. An unknown
   method is an error.
3. The parser turns the positional `params` into a typed `Subscribe`,
   `Authorize` or `Submit`.

A bad line always becomes an error to log. It never panics the server.

## References

- [Stratum V1 protocol](https://en.bitcoin.it/wiki/Stratum_mining_protocol)
- [Stratum V1 detailed reference](https://braiins.com/stratum-v1/docs)
- [Block hashing algorithm](https://en.bitcoin.it/wiki/Block_hashing_algorithm)
- [BIP 22](https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki) and
  [BIP 23](https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki):
  `getblocktemplate`
