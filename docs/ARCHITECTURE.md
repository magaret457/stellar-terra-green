# Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────┐
│                      SolarCert                          │
├──────────────────────┬──────────────────────────────────┤
│   Cooperative Panel  │        External Buyers           │
│   (Next.js Web App)  │    (API / Stellar Wallets)       │
└──────────┬───────────┴──────────────┬───────────────────┘
           │                          │
           ▼                          ▼
┌──────────────────────┐   ┌──────────────────────────────┐
│   Next.js API Routes │   │     Stellar Testnet           │
│   + Supabase DB      │──▶│   energy_token (SEP-41)      │
│                      │   │   energy_distribution         │
│   POST /api/meters/  │   │   community_governance        │
│         readings     │   └──────────────────────────────┘
└──────────────────────┘
           ▲
           │
┌──────────────────────┐
│    Smart Meters      │
│  (IoT / API push)    │
└──────────────────────┘
```

## Data Flow

1. **Meter Reading Ingestion**
   - Smart meter POSTs `{ meter_id, kwh, timestamp, api_key }` to `/api/meters/readings`
   - API validates the API key against the meter record in Supabase
   - Reading is persisted to `meter_readings` table
   - `mintCertificates()` is called → Stellar transaction mints `kwh * 1e7` tokens to the cooperative

2. **Certificate Retirement**
   - Buyer calls `POST /api/certificates/:id/retire` with their wallet address
   - API calls `retireCertificate()` → burns tokens on-chain
   - Certificate record is updated with `retired_at` and `buyer_address`

3. **Member Distribution**
   - Admin calls `energy_distribution.distribute(total_amount)` on-chain
   - Tokens are split proportionally by member shares

## Database Schema (Supabase)

```sql
-- Cooperatives
create table cooperatives (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  created_at timestamptz default now()
);

-- Members
create table members (
  id uuid primary key default gen_random_uuid(),
  cooperative_id uuid references cooperatives(id),
  name text not null,
  wallet_address text not null,
  joined_at timestamptz default now()
);

-- Meters
create table meters (
  id uuid primary key default gen_random_uuid(),
  cooperative_id uuid references cooperatives(id),
  serial_number text not null unique,
  location text,
  capacity_kw numeric,
  api_key text not null,
  active boolean default true
);

-- Meter Readings
create table meter_readings (
  id uuid primary key default gen_random_uuid(),
  meter_id uuid references meters(id),
  kwh numeric not null,
  timestamp timestamptz not null,
  created_at timestamptz default now()
);

-- Certificates
create table certificates (
  id uuid primary key default gen_random_uuid(),
  token_id text not null,
  amount_kwh numeric not null,
  issued_at timestamptz default now(),
  retired_at timestamptz,
  buyer_address text
);
```

## Monorepo

```
solarcert/
├── apps/
│   ├── contracts/           # Soroban smart contracts (Rust)
│   │   ├── energy_token/
│   │   ├── energy_distribution/
│   │   └── community_governance/
│   └── web/                 # Next.js 15 application
│       └── src/
│           ├── app/         # App Router pages + API routes
│           └── lib/         # Supabase + Stellar helpers
├── packages/
│   └── stellar/             # Shared Stellar utilities
├── docs/                    # Documentation
└── .github/
    ├── workflows/           # CI/CD
    └── ISSUE_TEMPLATE/
```
