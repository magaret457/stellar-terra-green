import { Contract, Networks, TransactionBuilder, BASE_FEE, Keypair } from '@stellar/stellar-sdk'
import { Server } from '@stellar/stellar-sdk/rpc'

const RPC_URL = 'https://soroban-testnet.stellar.org'
const NETWORK_PASSPHRASE = Networks.TESTNET

export async function retireCertificate({
  tokenId,
  buyerAddress,
}: {
  tokenId: string
  buyerAddress: string
}): Promise<string> {
  const server = new Server(RPC_URL)
  const adminKeypair = Keypair.fromSecret(process.env.STELLAR_ADMIN_SECRET_KEY!)
  const account = await server.getAccount(adminKeypair.publicKey())

  const contract = new Contract(process.env.NEXT_PUBLIC_ENERGY_TOKEN_CONTRACT_ID!)

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call('burn', ...[buyerAddress, tokenId] as Parameters<typeof contract.call>))
    .setTimeout(30)
    .build()

  tx.sign(adminKeypair)

  const result = await server.sendTransaction(tx)
  return result.hash
}
