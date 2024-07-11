import { Account, Keypair, Networks, Operation, SorobanDataBuilder, SorobanRpc, TransactionBuilder, xdr } from '@stellar/stellar-sdk'
import { assembleTransaction } from '@stellar/stellar-sdk/lib/soroban'
import { Horizon } from 'colorglyph-sdk'

const rpc = new SorobanRpc.Server('https://mainnet.stellar.validationcloud.io/v1/???')
// const rpc = new SorobanRpc.Server('https://soroban-testnet.stellar.org')
// const horizon = new Horizon.Server('https://horizon-testnet.stellar.org')
// const rpc = new SorobanRpc.Server('https://rpc-futurenet.stellar.org')
// const horizon = new Horizon.Server('https://horizon-futurenet.stellar.org')
const pubkey = 'GBDVX4VELCDSQ54KQJYTNHXAHFLBCA77ZY2USQBM4CSHTTV7DME7KALE'
// const pubkey = 'GDI2MBTG6FT6427QZO2T3Z2TBN4SNUP7YSSOJPU7N3ZMXFARXM4UJYPF' // SDRL5NX4JZGAFCKLMZT3FGU4KHLYXFQKOPXHEH2D3LQNYLORJLJ22DZM

const source = await rpc.getAccount(pubkey).then((res) => new Account(res.accountId(), res.sequenceNumber()))
const sequence = await rpc.getLatestLedger().then(({ sequence }) => sequence)

const wasm_hash = "ffd2249433d021c6a215e08d21b834ee985e92b8e6dccb816849033fa4523ab9"
// const wasm_hash = "390c0dc0c56b66db3847d1ab75d8bfda164bda5a7032a049921f5529202ddd95"
// // const contract_id = "CAMCM4WTWRM2UZBI7CEQE4HI2PZEAE3ZBDDNJTI3RB6CIZRJKRGXEHWK"

// // const wasm_path = '../target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm'
// // const wasm_file = await Bun.file(wasm_path)
// // const wasm = Buffer.from(await wasm_file.arrayBuffer())

const lk = xdr.LedgerKey.contractCode(
    new xdr.LedgerKeyContractCode({
        hash: Buffer.from(wasm_hash, 'hex'),
    })
)
const le = await rpc.getLedgerEntries(lk)

console.log(
    sequence,
    le.entries[0].liveUntilLedgerSeq!
);

const txn = new TransactionBuilder(source, {
    fee: '10000',
    networkPassphrase: Networks.PUBLIC
    // networkPassphrase: Networks.TESTNET
    // networkPassphrase: Networks.FUTURENET
})
    .addOperation(Operation.extendFootprintTtl({
        extendTo: 3110400 - 1
    }))
    .setSorobanData(
        new SorobanDataBuilder()
            .setReadOnly([
                // xdr.LedgerKey.contractData(
                //     new xdr.LedgerKeyContractData({
                //         contract: Address.fromString(contract_id).toScAddress(),
                //         key: xdr.ScVal.scvLedgerKeyContractInstance(),
                //         durability: xdr.ContractDataDurability.persistent()
                //     })
                // ),
                lk
            ])
            .build()
    )
    .setTimeout(0)
    .build()

const simTxn = await rpc.simulateTransaction(txn)

if (!SorobanRpc.Api.isSimulationSuccess(simTxn)) {
    throw new Error('Simulation failed')
}

const tx = assembleTransaction(txn, simTxn).build()

// tx.sign(Keypair.fromSecret('SDRL5NX4JZGAFCKLMZT3FGU4KHLYXFQKOPXHEH2D3LQNYLORJLJ22DZM'))

console.log(tx.fee);
console.log(tx.toXDR());

// const res = await horizon.submitTransaction(tx)

// console.log(res);
