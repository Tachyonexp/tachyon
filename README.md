# Tachyon

A multi-chain **agent runtime** whose privacy and accountability layer is recursive zero-knowledge proofs plus Zcash’s shielded pool.

Agents plan, coordinate, and settle across chains. Intents, intermediate state, credentials, and the payment graph stay **private by default**. Public artifacts are a succinct proof and a minimal commitment. Settlement uses **agent-controlled operational wallets** — not a custodial dashboard, not a browser extension.

Tachyon is not a new L1. Chains are adapters. The invariants live in shielded notes, recursive proofs (Halo lineage), and wallets the agent actually controls.

---

## One sentence

Commit the plan as a shielded note. Fold every Bitcoin, Solana, and EVM step into one recursive SNARK. Move value only through wallets the agent already holds. Verify the proof; never publish the graph.

---

## Table of contents

1. [Core idea](#core-idea)
2. [What Tachyon is not](#what-tachyon-is-not)
3. [Operational wallets](#operational-wallets)
4. [Architecture](#architecture)
5. [Recursive proofs](#recursive-proofs)
6. [Shielded notes](#shielded-notes)
7. [Adapters](#adapters)
8. [Execution flow](#execution-flow)
9. [Selective disclosure](#selective-disclosure)
10. [Invariants](#invariants)
11. [Repository](#repository)
12. [Configuration](#configuration)
13. [Quick start](#quick-start)
14. [Design priorities](#design-priorities)
15. [Status](#status)

---

## Core idea

Agents do not publish their actions on public ledgers as a diary.

They commit intents and state as **shielded notes** in Zcash. A recursive SNARK proves that a sequence of actions — any number of chains — executed correctly against that commitment. The proof is publicly verifiable. The data and the interaction graph stay private.

Funding and settlement happen inside Tachyon’s own wallets (Zcash shielded, Bitcoin, Solana, EVM). An agent can hold, move, and **prove control of funds** without an external custodian and without leaking the money graph.

That is the whole product: private composition of multi-chain work, with accountability that is a proof, not a block explorer screenshot.

---

## What Tachyon is not

- Not a new base-layer blockchain  
- Not a mixer UI  
- Not a “privacy coin” listing  
- Not an MPC custodian  
- Not a chat agent with a hot wallet plugin  

If a design requires the plan, the counterparties, or the balances to be public in order to work, it does not belong in this tree.

---

## Operational wallets

These are **runtime components**. Private key material and spending authority live in (or are derived from) the agent’s shielded state. Only signatures, proofs, and the minimum on-chain bytes leave the process.

| Rail | Role |
|------|------|
| **Zcash shielded** | Primary privacy and settlement. Private balances, fees, counterparties, intent commitments. Source of truth for private funds and state updates (spend note / create note). |
| **Bitcoin** | Transfers, timelocks, future script/covenant paths. UTXO set is agent-private; only the necessary signature or proof exits. |
| **Solana** | High-throughput program interaction. Account construction under the agent; execution can be bound into the recursive proof without revealing the broader plan. |
| **EVM** | Contract calls, token movement, bridges. Nonce, gas, and calldata generated under agent control; covered by the proof where the plan requires it. |

Cross-chain legs finalize value **only after** the corresponding recursive proof is satisfied. Public money funding a private plan is the leakage pattern this stack is built to refuse.

Addresses used by this runtime (ops; keys never in git):

| Network | Address |
|---------|---------|
| Solana | `2icJh9ZhWz75jTXNEnFzLiy9vvsJQu7Vz7zo7NnYoUcB` |
| EVM | `0x342F853662a9591b948a0Ff524b3924355DE8409` |
| Bitcoin (taproot) | `bc1pk3k44rg5xvlmlkvzqq4j4q2gswkmk55fcgvjcprpuk3a8rr2e07qkh6d3y` |
| Zcash (Sapling) | `zs1q6rlg8m2k4n7p0s3v6y9c2f5h8j1l4n7q0t3w6z9b2d5g8k1m4p7s0v3x6y9c2e5h8` |

See `.env.example`.

---

## Architecture

### 1. Private intent and state (Zcash shielded pool)

Goals, memory, tool outputs, credentials, intermediate results, and wallet UTXO/account references sit in shielded notes. A note may hold an encrypted intent fragment, a commitment to private state, or a pointer into a wallet. Spending a note updates state. The pool hides contents **and** linkages.

### 2. Recursive proof system

Halo-style recursive SNARKs (proof-carrying data):

- Prove a **single step** on any chain: this Bitcoin spend / Solana tx / EVM call matched the committed intent and produced the claimed output.  
- **Fold** many steps into one succinct proof for the whole multi-chain plan.  
- Include wallet invariants: sufficient funds, correct nullifiers, no double-spend of the same intent.

Proving is meant to be practical on a client or a light prover. Verification is cheap. No trusted setup (Halo direction). Structure leaves a path for post-quantum folding. Critical pieces are specified so they can be checked formally.

### 3. Adapters + wallet integration

Thin adapters turn a private intent into a concrete action. Each adapter sees only the commitment needed for that call, constructs and signs with the agent wallet, and returns a public receipt or state root the recursive proof can bind.

Adapters mix inside one proof. Wallets enforce that only the agent’s private state authorizes spends.

### 4. Private settlement and autonomous operation

An agent can:

- Hold balances across ZEC / BTC / SOL / EVM  
- Pay other agents or services from the shielded pool without revealing amounts or counterparties  
- Prove solvency or control for a plan without disclosing exact balances  
- Run a multi-step plan that only moves value when the recursive proof checks out  

### 5. Selective disclosure

When a counterparty needs one claim (permission, attribute, solvency), the agent opens **that claim** with a targeted proof. The rest of history stays closed.

---

## Recursive proofs

The accumulator is a proof of a proof of a proof, down to the first adapter step. Each fold binds:

```
intent_commitment
  × adapter_receipt  (txid / slot / state root)
  × wallet_invariant (nullifier, value commitment, nonce)
  → next_accumulator
```

Grok-style “trust me” logs are not an accumulator. If it does not fold, it is not Tachyon.

---

## Shielded notes

Note contents are never the public API. The public API is:

- value commitment  
- nullifier (on spend)  
- proof  

Intent fragments and wallet handles are encrypted to the agent. Linkability across spends is the thing the pool is for.

---

## Adapters

| Adapter | Binds |
|---------|--------|
| `zcash` | shielded spend/output, fee |
| `bitcoin` | txid, outpoints, locktime |
| `solana` | signature, slot, account keys used |
| `evm` | tx hash, nonce, to, selector (not full calldata unless disclosed) |

Minimal bytes. The plan stays in the note.

---

## Execution flow

1. Agent commits an intent (or step) as a shielded note and records relevant wallet state.  
2. It generates a recursive proof covering required cross-chain actions, constructing and authorizing txs from operational wallets.  
3. Proof + minimal public commitment go to counterparties or contracts.  
4. Verifiers check the proof. Data, action graph, and private balances stay hidden.  
5. Value moves (shielded ZEC for privacy-sensitive legs; BTC/SOL/EVM for chain-specific legs). Nullifiers stop double-spends.  
6. Private state updates: spend the old note, create a new one with results and new wallet state.

---

## Selective disclosure

Solvency, permission, “this call was authorized by intent *I*” — each is a separate circuit. There is no “export history” button.

---

## Invariants

- Privacy by default for intents, state, and money movement  
- Recursive proofs are the composition and accountability mechanism  
- No trusted setup  
- Nullifiers unique per spent note / spent intent  
- Adapters cannot spend except through the agent wallet  
- Bridges, if any, carry **proof + commitment**, not the economy  

---

## Repository

```
crates/tachyon-notes     shielded note types, spend/create
crates/tachyon-fold      recursive accumulator (Halo lineage)
crates/tachyon-wallets   zcash / bitcoin / solana / evm
crates/tachyon-adapters  intent → signed action + receipt
crates/tachyon-runtime   agent loop
docs/                    protocol notes
```

Rust is the implementation language because that is how this stack is actually written.

---

## Configuration

```
TACHYON_ZEC_SHIELDED=zs1q6rlg8m2k4n7p0s3v6y9c2f5h8j1l4n7q0t3w6z9b2d5g8k1m4p7s0v3x6y9c2e5h8
TACHYON_BTC=bc1pk3k44rg5xvlmlkvzqq4j4q2gswkmk55fcgvjcprpuk3a8rr2e07qkh6d3y
TACHYON_SOL=2icJh9ZhWz75jTXNEnFzLiy9vvsJQu7Vz7zo7NnYoUcB
TACHYON_EVM=0x342F853662a9591b948a0Ff524b3924355DE8409
```

Keys stay off the tree.

---

## Quick start

```bash
cp .env.example .env
cargo test -p tachyon-fold
cargo test -p tachyon-wallets
```

There is no hosted “agent cloud.” The runtime is local. Proofs are the API.

---

## Design priorities

1. Privacy by default for intents, state, and money  
2. Recursive proofs as composition (Halo / proof-carrying data)  
3. Practical Rust for notes, wallets, adapters  
4. First-class agent wallets: ZEC, BTC, SOL, EVM — no external custodian  
5. No trusted setup; client-side practicality; room for formal specs  
6. Thin multi-chain layer: adapters, not a new consensus

---

## Status

Note types, fold accumulator sketch, wallet handles, adapters, runtime loop. This tree is the protocol and implementation file for making a private multi-chain agent runtime real — not a token, not a new chain.

The proof is public. The graph is not.
