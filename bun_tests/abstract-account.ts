const COLORGLYPH_ID = 'CD7KB6OJFFDT6O3SD2DCLN54X7WHGHBRFPY6QWIC3TXGMDWP35ZNHCAF'
const ACCOUNT_ID = 'CC3JJRDTKYB2OHKGTOLYW2MPZIYGYESUR5TGSIEOLW4AS6CT2O5V7G36'

import { xdr, Account, Keypair, Networks, Operation, SorobanRpc, TransactionBuilder, nativeToScVal, Address } from '@stellar/stellar-sdk'
import { assembleTransaction } from '@stellar/stellar-sdk/lib/soroban'

const rpc = new SorobanRpc.Server('http://localhost:8000/soroban/rpc', { allowHttp: true })
const secret = 'SA7E3D73763HEN2GNHOLNJUH3EQWLN34NRRSEKFJFSU7ENJICRGL35F6'
const keypair = Keypair.fromSecret(secret)
const pubkey = keypair.publicKey()

const source = await rpc.getAccount(pubkey).then((res) => new Account(res.accountId(), res.sequenceNumber()))

const simTxn = new TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: Networks.STANDALONE
})
    .addOperation(Operation.invokeContractFunction({
        contract: COLORGLYPH_ID,
        function: 'colors_mine',
        args: [
            nativeToScVal(ACCOUNT_ID, { type: 'address' }),
            xdr.ScVal.scvMap([
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvU32(0),
                    val: xdr.ScVal.scvU32(1000)
                }),
            ]),
            xdr.ScVal.scvVoid(),
            xdr.ScVal.scvVoid(),
        ]
    }))
    .setTimeout(0)
    .build()

let simRes = await rpc._simulateTransaction(simTxn)

// if (SorobanRpc.Api.isSimulationSuccess(simRes)) {
//     simRes.transactionData.appendFootprint([
//         xdr.LedgerKey.contractData(
//             new xdr.LedgerKeyContractData({
//                 contract: Address.fromString(ACCOUNT_ID).toScAddress(),
//                 key: xdr.ScVal.scvLedgerKeyContractInstance(),
//                 durability: xdr.ContractDataDurability.persistent()
//             })
//         ),
//     ], [])
// }

console.log(simRes);

const authTxn = assembleTransaction(simTxn, simRes).build()

for (const op of authTxn.operations) {
    const auths = (op as Operation.InvokeHostFunction).auth

    if (!auths?.length)
        continue;

    for (let i = 0; i < auths.length; i++) {
        const { sequence } = await rpc.getLatestLedger()
        const creds = auths[i].credentials().address()

        auths[i] = new xdr.SorobanAuthorizationEntry({
            credentials: xdr.SorobanCredentials.sorobanCredentialsAddress(
                new xdr.SorobanAddressCredentials({
                    address: creds.address(),
                    nonce: creds.nonce(),
                    signatureExpirationLedger: sequence + 12,
                    signature: xdr.ScVal.scvMap([
                        new xdr.ScMapEntry({
                          key: xdr.ScVal.scvSymbol('public_key'),
                          val: xdr.ScVal.scvBytes(Buffer.alloc(32, 0)),
                        }),
                        new xdr.ScMapEntry({
                          key: xdr.ScVal.scvSymbol('signature'),
                          val: xdr.ScVal.scvBytes(Buffer.alloc(64, 0)),
                        }),
                      ])
                })
            ),
            rootInvocation: auths[i].rootInvocation(),
        })
    }
}

console.log(authTxn.toXDR());

simRes = await rpc._simulateTransaction(authTxn)

console.log(simRes);

const readyTxn = assembleTransaction(authTxn, simRes).build()

readyTxn.sign(keypair)

console.log(readyTxn.toXDR());

const sendTxn = await rpc._sendTransaction(readyTxn)

await Bun.sleep(5000)

const getTxn = await rpc._getTransaction(sendTxn.hash)

console.log(getTxn);

