import { Account, Horizon, Keypair, Networks, Operation, SorobanRpc, TransactionBuilder, hash } from '@stellar/stellar-sdk'
import { assembleTransaction } from '@stellar/stellar-sdk/lib/soroban'

// const rpc = new SorobanRpc.Server('https://mainnet.stellar.validationcloud.io/v1/???')
const rpc = new SorobanRpc.Server('https://soroban-testnet.stellar.org')
const horizon = new Horizon.Server('https://horizon-testnet.stellar.org')
// const rpc = new SorobanRpc.Server('https://rpc-futurenet.stellar.org')
// const pubkey = 'GBDVX4VELCDSQ54KQJYTNHXAHFLBCA77ZY2USQBM4CSHTTV7DME7KALE'
const pubkey = 'GDI2MBTG6FT6427QZO2T3Z2TBN4SNUP7YSSOJPU7N3ZMXFARXM4UJYPF' // SDRL5NX4JZGAFCKLMZT3FGU4KHLYXFQKOPXHEH2D3LQNYLORJLJ22DZM

const source = await rpc.getAccount(pubkey).then((res) => new Account(res.accountId(), res.sequenceNumber()))

const wasm_path = '../target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm'
const wasm_file = await Bun.file(wasm_path)
const wasm = Buffer.from(await wasm_file.arrayBuffer())

console.log(hash(wasm).toString('hex'));

const txn = new TransactionBuilder(source, {
    fee: (10_000_000).toString(),
    // networkPassphrase: Networks.PUBLIC
    networkPassphrase: Networks.TESTNET
    // networkPassphrase: Networks.FUTURENET
})
.addOperation(Operation.uploadContractWasm({
    wasm
}))
.setTimeout(0)
.build()

const simTxn = await rpc.simulateTransaction(txn)

if (!SorobanRpc.Api.isSimulationSuccess(simTxn)) {
    throw new Error('Simulation failed')
}

const tx = assembleTransaction(txn, simTxn).build()

tx.sign(Keypair.fromSecret('SDRL5NX4JZGAFCKLMZT3FGU4KHLYXFQKOPXHEH2D3LQNYLORJLJ22DZM'))

console.log(tx.fee);
console.log(tx.toXDR());

const res = await horizon.submitTransaction(tx)

console.log(res);