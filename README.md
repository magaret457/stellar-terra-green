# SolarCert

**Renewable energy certification infrastructure on Stellar**

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Built on Stellar](https://img.shields.io/badge/built%20on-Stellar-7B2FBE)](https://stellar.org)
[![Deployed on Vercel](https://img.shields.io/badge/deployed%20on-Vercel-black)](https://vercel.com)
[![Stellar Testnet](https://img.shields.io/badge/network-Stellar%20Testnet-blueviolet)](https://stellar.org/developers)

SolarCert is a cooperative management dashboard and on-chain certification infrastructure for renewable energy on Stellar. Cooperatives use the dashboard to manage members, meters, readings, and statistics — and SolarCert tokenizes their production as proto-certificates sold to external buyers (companies, ESG funds, climate programs).

---

## What it does

Two things:

1. **Cooperative management dashboard** — Web panel where cooperatives manage members, meters, generation statistics, and certificates.
2. **On-chain certification** — Tokenizes renewable production as proto-certificates on Stellar (1 token = 1 kWh), sold to external buyers.

```
Smart meter sends readings via API (POST /api/meters/readings)
        |
SolarCert mints proto-certificates on-chain (1 token = 1 kWh)
        |
External buyer purchases certificates
        |
Buyer retires certificate (burn on-chain)
```

Each token is a **proto-certificate**: a verifiable on-chain claim that 1 kWh of renewable energy was generated. It is not a financial instrument, not electricity for consumption, and not designed for trading between members.

---

## Live Demo

- **Frontend:** https://solarcert.vercel.app
- **Network:** Stellar Testnet

---

## Smart Contracts (Soroban)

Deployed on Stellar Testnet:

| Contract | Purpose |
|---|---|
| `energy_token` | SEP-41 fungible token representing renewable generation certificates |
| `energy_distribution` | Allocates certificates to cooperative members by participation |
| `community_governance` | Cooperative governance (proposals) |

Built with **OpenZeppelin Stellar v0.5.1** (Pausable + Upgradeable) and **Soroban SDK 23.1.0**.

65 tests passing. See [`docs/CONTRACTS.md`](docs/CONTRACTS.md) for full reference.

---

## Monorepo Structure

```
solarcert/
├── apps/
│   ├── contracts/           # Soroban smart contracts (Rust)
│   │   ├── energy_token/
│   │   ├── energy_distribution/
│   │   └── community_governance/
│   └── web/                 # Next.js application
├── packages/
│   └── stellar/             # Shared Stellar utilities
├── docs/                    # Documentation
└── tooling/
    └── github/              # GitHub issue templates & workflows
```

Powered by **Turborepo + pnpm workspaces**.

---

## Quick Start

### Prerequisites

- Node.js v22+
- pnpm v10+ (`corepack enable`)
- Rust + Cargo (for contracts)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli)

### Installation

```bash
git clone https://github.com/your-org/solarcert.git
cd solarcert
pnpm install
```

### Run Development Server

```bash
pnpm dev
```

Frontend: http://localhost:3000

### Build and Test Contracts

```bash
cd apps/contracts
stellar contract build
cargo test
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Blockchain | Stellar (Soroban smart contracts) |
| Smart Contracts | Rust + OpenZeppelin Stellar v0.5.1 |
| Frontend | Next.js 15 + React 19 + TypeScript |
| Styling | Tailwind CSS v4 + shadcn/ui |
| Wallet | Freighter + Stellar Wallets Kit |
| Backend | Next.js API Routes + Supabase |
| Deployment | Vercel |
| Monorepo | Turborepo + pnpm |

---

## Contributing

1. Browse [open issues](https://github.com/your-org/solarcert/issues)
2. Comment to claim an issue
3. Fork, code, and submit a PR to `develop`

### Branch Structure

- `main` — Production (protected)
- `develop` — Active development

```bash
git clone https://github.com/YOUR-USERNAME/solarcert.git
cd solarcert
git checkout develop
git checkout -b feat/your-feature
pnpm install && pnpm dev
# Submit PR to develop branch
```

---

## Product Levels

| Level | What | Status |
|---|---|---|
| 1 — Internal registry | Token = production record for cooperatives | ✅ Current |
| 2 — Verifiable certification | Smart meters + oracles + independent verification | 🔜 Next |
| 3 — Recognized standard | Integration with I-REC, Energy Web, TIGR | 🔮 Future |

---

## License

Apache-2.0 — See [LICENSE](LICENSE) for details.

---

*Built on Stellar | SolarCert Contributors 2026*
