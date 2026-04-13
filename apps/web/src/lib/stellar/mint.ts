import { Contract, Networks, TransactionBuilder, BASE_FEE, Keypair } from '@stellar/stellar-sdk'
import { Server } from '@stellar/stellar-sdk/rpc'

const RPC_URL = 'https://soroban-testnet.stellar.org'
const NETWORK_PASSPHRASE = Networks.TESTNET

export async function mintCertificates({
  cooperativeId,
  kwh,
}: {
  cooperativeId: string
  kwh: number
}): Promise<string> {
  const server = new Server(RPC_URL)
  const adminKeypair = Keypair.fromSecret(process.env.STELLAR_ADMIN_SECRET_KEY!)
  const account = await server.getAccount(adminKeypair.publicKey())

  const contract = new Contract(process.env.NEXT_PUBLIC_ENERGY_TOKEN_CONTRACT_ID!)

  // Amount in stroops-equivalent: 1 token = 1 kWh, 7 decimal places
  const amount = BigInt(Math.round(kwh * 1e7))

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call('mint', ...[cooperativeId, amount] as Parameters<typeof contract.call>))
    .setTimeout(30)
    .build()

  tx.sign(adminKeypair)

  const result = await server.sendTransaction(tx)
  return result.hash
}
