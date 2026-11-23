# Build stages

Last updated: Nov 22, 2026, at commit `2fd8a42`.

The pool is built in stages, each one small enough to finish and test before
the next begins. Update the status and checklists here as work lands.

| Stage | What | Status |
| --- | --- | --- |
| 1 | Project setup | Done |
| 2 | Stratum message codec | Done |
| 3 | TCP listener | In progress |
| 4 | Share validation | Not started |
| 5 | Bitcoin Core integration | Not started |
| 6 | Job management | Not started |
| 7 | Payouts (PPLNS) | Not started |
| 8 | Real hardware on testnet4 | Not started |
| 9 | Dashboard and operations | Not started |
| 10 | Mainnet | Not started |

## Stage 1: Project setup

**Status: Done**

- [x] `cargo init`, git repository
- [x] Dependencies: `serde` (with `derive`) and `serde_json`

## Stage 2: Stratum message codec

**Status: Done**

Turn a line of text from a miner into a typed value, and turn the pool's
replies back into lines.

- [x] Loose `Request` that accepts any JSON-RPC request: number, string or null
      id; positional `params`
- [x] Reject malformed lines with an error: missing `params`, non-JSON input
- [x] Typed `mining.subscribe` (optional user agent)
- [x] Typed `mining.authorize` (required worker name, optional password)
- [x] Typed `mining.submit` (worker name, job id, extranonce2, ntime, nonce)
- [x] Shared helpers for required and optional text fields
- [x] `parse_message`: choose the parser from `method`; unknown methods are an
      error
- [x] `Response` serialization (`id`, `result`, `error`)
- [x] Notification serialization (`mining.set_difficulty` shape)
- [x] 22 tests, in `src/stratum/tests.rs`

Left for later stages:

- [ ] Typed `mining.notify` builder (needs a block template; stages 4 and 5)
- [ ] `mining.configure` and version rolling (stage 8)
- [ ] Decode and validate the hex fields in `mining.submit` (stage 4)
- [ ] A typed error with Stratum error codes, replacing `String`

## Stage 3: TCP listener

**Status: In progress**

Accept a miner's connection and hold a conversation with it.

- [x] Listen on `127.0.0.1:3333` and accept one connection
- [x] Read the connection line by line
- [x] Parse each line and log the typed message
- [x] Log bad lines as errors and keep the connection going
- [x] Reply to each message with a `Response` that echoes its id
- [x] Reply per message type; `mining.subscribe` returns `extranonce1` and the
      `extranonce2` size
- [ ] Send `mining.set_difficulty` after authorize
- [ ] Accept connections in a loop
- [ ] Handle several connections at once (move to tokio)
- [ ] Limit the length of a line
- [ ] Capture traffic from a real miner

## Stage 4: Share validation

**Status: Not started**

Check that a submitted share is real work.

- [ ] Rebuild the coinbase from its two halves, `extranonce1` and `extranonce2`
- [ ] Compute the merkle root from the coinbase hash and the merkle branches
- [ ] Assemble the 80-byte block header
- [ ] Double SHA-256 the header
- [ ] Compare the hash with the share target and the network target
- [ ] Reject stale and duplicate shares
- [ ] Known-block tests: reproduce the hash of real mainnet blocks

## Stage 5: Bitcoin Core integration

**Status: Not started**

Get real work from a node and hand found blocks back to it.

- [ ] Bitcoin Core running on regtest
- [ ] RPC client: `getblocktemplate`, `submitblock`
- [ ] Build the coinbase transaction: block height (BIP 34), message, payout
      outputs, witness commitment (BIP 141)
- [ ] Build `mining.notify` jobs from a template
- [ ] Validate each built block with `getblocktemplate` in proposal mode
- [ ] Mine a regtest block through the pool

## Stage 6: Job management

**Status: Not started**

- [ ] New job on every new block (ZMQ `hashblock`), with `clean_jobs`
- [ ] Refresh the template on a timer to pick up fees
- [ ] Per-miner variable difficulty

## Stage 7: Payouts (PPLNS)

**Status: Not started**

- [ ] Store accepted shares with their difficulty
- [ ] Compute the payout split from the recent window of shares
- [ ] Write the split as coinbase outputs, with a minimum payout

## Stage 8: Real hardware on testnet4

**Status: Not started**

- [ ] `mining.configure` and version rolling (BIP 310)
- [ ] Real miners run for a week; every block found is accepted

## Stage 9: Dashboard and operations

**Status: Not started**

- [ ] Web page: hashrate, shares, payout split, blocks found
- [ ] Run as a service, with an alert when the node or pool stops

## Stage 10: Mainnet

**Status: Not started**

- [ ] One miner first, with the first jobs watched by hand
- [ ] Add the second miner
