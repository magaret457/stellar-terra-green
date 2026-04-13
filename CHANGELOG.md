# Changelog

All notable changes to SolarCert are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased]

### Added
- Initial monorepo setup with Turborepo + pnpm workspaces
- `energy_token` Soroban contract (SEP-41, mint/burn/transfer)
- `energy_distribution` Soroban contract (proportional member allocation)
- `community_governance` Soroban contract (proposals + voting)
- Next.js 15 web application with App Router
- Cooperative dashboard (members, meters, statistics)
- Certificate browser and retirement UI
- `POST /api/meters/readings` — smart meter ingestion endpoint
- `POST /api/certificates/:id/retire` — on-chain certificate retirement
- `@solarcert/stellar` shared package (network helpers, formatters)
- Supabase integration (server + browser clients)
- GitHub Actions: CI, Vercel deploy, contract deploy workflows
- Issue templates: bug report, feature request, good first issue
- PR template
- Docs: CONTRACTS.md, ARCHITECTURE.md, DEPLOYMENT.md
- CONTRIBUTING.md and SECURITY.md

---

## [0.1.0] — 2026-04-13

Initial release on Stellar Testnet.
