# Deployment Guide

## Frontend (Vercel)

1. Import the repo into [Vercel](https://vercel.com)
2. Set root directory to `apps/web`
3. Add environment variables from `apps/web/.env.example`
4. Deploy — Vercel auto-detects Next.js

### Required Environment Variables

| Variable | Description |
|---|---|
| `NEXT_PUBLIC_SUPABASE_URL` | Supabase project URL |
| `NEXT_PUBLIC_SUPABASE_ANON_KEY` | Supabase anon key |
| `SUPABASE_SERVICE_ROLE_KEY` | Supabase service role (server only) |
| `NEXT_PUBLIC_STELLAR_NETWORK` | `testnet` or `mainnet` |
| `NEXT_PUBLIC_ENERGY_TOKEN_CONTRACT_ID` | Deployed contract address |
| `NEXT_PUBLIC_ENERGY_DISTRIBUTION_CONTRACT_ID` | Deployed contract address |
| `NEXT_PUBLIC_COMMUNITY_GOVERNANCE_CONTRACT_ID` | Deployed contract address |
| `STELLAR_ADMIN_SECRET_KEY` | Admin keypair secret (never expose to client) |

---

## Smart Contracts (Stellar Testnet)

### Prerequisites

```bash
# Install Stellar CLI
cargo install --locked stellar-cli --features opt

# Configure testnet identity
stellar keys generate --global deployer --network testnet
stellar keys fund deployer --network testnet  # fund from friendbot
```

### Build

```bash
cd apps/contracts
stellar contract build
```

### Deploy

```bash
# Deploy energy_token
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/energy_token.wasm \
  --source deployer \
  --network testnet

# Deploy energy_distribution
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/energy_distribution.wasm \
  --source deployer \
  --network testnet

# Deploy community_governance
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/community_governance.wasm \
  --source deployer \
  --network testnet
```

### Initialize Contracts

```bash
# Initialize energy_token
stellar contract invoke \
  --id <ENERGY_TOKEN_CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --minter <MINTER_ADDRESS> \
  --name "SolarCert Energy Token" \
  --symbol "SCERT"
```

---

## CI/CD

- **CI** runs on every push/PR to `main` and `develop` (see `.github/workflows/ci.yml`)
- **Deploy to Vercel** runs on push to `main` (see `.github/workflows/deploy.yml`)
- **Deploy Contracts** is manual via workflow dispatch (see `.github/workflows/deploy-contracts.yml`)

### Required GitHub Secrets

| Secret | Used by |
|---|---|
| `VERCEL_TOKEN` | deploy.yml |
| `VERCEL_ORG_ID` | deploy.yml |
| `VERCEL_PROJECT_ID` | deploy.yml |
| `STELLAR_DEPLOY_SECRET_KEY` | deploy-contracts.yml |
