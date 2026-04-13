# Contributing to SolarCert

Thank you for your interest in contributing! SolarCert is an open-source project and we welcome contributions of all kinds.

---

## Getting Started

1. Browse [open issues](https://github.com/your-org/solarcert/issues)
2. Comment on an issue to claim it — wait for a maintainer to assign it to you
3. Fork the repo and create a branch from `develop`
4. Submit a PR back to `develop`

---

## Development Setup

### Prerequisites

- Node.js v22+
- pnpm v10+ (`corepack enable`)
- Rust + Cargo
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli)

### Install

```bash
git clone https://github.com/YOUR-USERNAME/solarcert.git
cd solarcert
git checkout develop
git checkout -b feat/your-feature
pnpm install
```

### Run

```bash
pnpm dev          # starts Next.js at localhost:3000
```

### Environment

Copy `apps/web/.env.example` to `apps/web/.env.local` and fill in values.

---

## Branch Conventions

| Branch | Purpose |
|---|---|
| `main` | Production — protected, no direct pushes |
| `develop` | Active development — PRs target here |
| `feat/*` | New features |
| `fix/*` | Bug fixes |
| `docs/*` | Documentation only |
| `chore/*` | Tooling, deps, CI |

---

## Commit Style

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add meter reading chart
fix: correct token decimal precision
docs: update contract reference
chore: bump soroban-sdk to 23.1.0
```

---

## Code Standards

- **TypeScript**: strict mode, no `any`
- **Rust**: `cargo clippy` must pass, no warnings
- **Formatting**: `pnpm format` (Prettier)
- **Linting**: `pnpm lint`

---

## Smart Contract Contributions

Contracts live in `apps/contracts/`. Each contract has its own `Cargo.toml`.

```bash
cd apps/contracts
cargo test --all        # run all tests
stellar contract build  # compile to WASM
```

All new contract logic must have tests. We aim for >80% coverage.

---

## Questions?

Open a [Discussion](https://github.com/your-org/solarcert/discussions) or comment on the relevant issue.
