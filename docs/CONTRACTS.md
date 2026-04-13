# Smart Contract Reference

## Overview

SolarCert deploys three Soroban contracts on Stellar Testnet:

| Contract | Purpose |
|---|---|
| `energy_token` | SEP-41 fungible token — 1 token = 1 kWh |
| `energy_distribution` | Proportional allocation to cooperative members |
| `community_governance` | On-chain proposal and voting system |

All contracts are built with **OpenZeppelin Stellar v0.5.1** and **Soroban SDK 23.1.0**.

---

## energy_token

### Interface

| Function | Auth | Description |
|---|---|---|
| `initialize(admin, minter, name, symbol)` | — | One-time setup |
| `mint(to, amount)` | minter | Mint tokens (1 = 1 kWh) |
| `burn(from, amount)` | from | Retire a certificate |
| `transfer(from, to, amount)` | from | Transfer tokens |
| `balance(id)` | — | Get token balance |
| `set_admin(new_admin)` | admin | Transfer admin role |
| `set_minter(new_minter)` | admin | Transfer minter role |

### Token Metadata

- **Name:** SolarCert Energy Token
- **Symbol:** SCERT
- **Decimals:** 7

---

## energy_distribution

### Interface

| Function | Auth | Description |
|---|---|---|
| `initialize(admin, token_contract)` | — | One-time setup |
| `set_shares(members, shares)` | admin | Set participation weights |
| `distribute(total_amount)` | admin | Distribute tokens proportionally |
| `get_share(member)` | — | Get a member's share weight |

### Share Calculation

Shares are relative weights. Example: Alice=60, Bob=40 → Alice gets 60%, Bob gets 40% of any distribution.

---

## community_governance

### Interface

| Function | Auth | Description |
|---|---|---|
| `initialize(admin)` | — | One-time setup |
| `propose(proposer, title, description)` | proposer | Submit a proposal |
| `vote(voter, proposal_id, approve)` | voter | Cast a vote |
| `finalize(proposal_id)` | admin | Close voting and set outcome |
| `get_proposal(proposal_id)` | — | Fetch proposal details |
| `proposal_count()` | — | Total proposals submitted |

### Proposal Lifecycle

```
propose() → Active
              ↓
           vote() (multiple)
              ↓
           finalize() → Approved | Rejected
```

---

## Building & Testing

```bash
cd apps/contracts

# Run all tests
cargo test --all

# Build WASM
stellar contract build

# Deploy to testnet (requires Stellar CLI configured)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/energy_token.wasm \
  --source <your-secret-key> \
  --network testnet
```

---

## Test Coverage

65 tests across all three contracts. Run with:

```bash
cargo test --all -- --nocapture
```
