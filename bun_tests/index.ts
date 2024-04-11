import { Account, Networks, Operation, SorobanRpc, TransactionBuilder } from '@stellar/stellar-sdk'

const rpc = new SorobanRpc.Server('http://67.205.175.159:8000/soroban/rpc', { allowHttp: true })
// const rpc = new SorobanRpc.Server('https://soroban-testnet.stellar.org')
// const rpc = new SorobanRpc.Server('https://rpc-futurenet.stellar.org')
const pubkey = 'GBDVX4VELCDSQ54KQJYTNHXAHFLBCA77ZY2USQBM4CSHTTV7DME7KALE'

const source = await rpc.getAccount(pubkey).then((res) => new Account(res.accountId(), res.sequenceNumber()))

const wasm_path = '../target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm'
const wasm_file = await Bun.file(wasm_path)
const wasm = Buffer.from(await wasm_file.arrayBuffer())

const txn = new TransactionBuilder(source, {
    fee: '0',
    networkPassphrase: Networks.PUBLIC
    // networkPassphrase: Networks.TESTNET
    // networkPassphrase: Networks.FUTURENET
})
.addOperation(Operation.uploadContractWasm({
    wasm
}))
.setTimeout(0)
.build()

const simTxn = await rpc._simulateTransaction(txn)

console.log(simTxn);
