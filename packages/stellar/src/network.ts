import { Networks } from '@stellar/stellar-sdk'

export type StellarNetwork = 'testnet' | 'mainnet'

export const NETWORKS: Record<StellarNetwork, string> = {
  testnet: Networks.TESTNET,
  mainnet: Networks.PUBLIC,
}

export function getRpcUrl(network: StellarNetwork): string {
  return network === 'mainnet'
    ? 'https://soroban.stellar.org'
    : 'https://soroban-testnet.stellar.org'
}
