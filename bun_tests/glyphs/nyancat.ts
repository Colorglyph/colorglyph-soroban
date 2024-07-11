import { Client } from 'colorglyph-sdk'
import type { Glyph } from 'colorglyph-sdk'
import { Keypair, Networks, Transaction } from '@stellar/stellar-sdk'

// const contractId = "CAMCM4WTWRM2UZBI7CEQE4HI2PZEAE3ZBDDNJTI3RB6CIZRJKRGXEHWK"
const contractId = "CDVRYQZQGWAZQOKN4AJNR4ZSTXXRR3VYXITZLTDNW3YGOTO4N3DVVKB4"
// const networkPassphrase = Networks.PUBLIC
const networkPassphrase = Networks.TESTNET
// const rpcUrl = Bun.env.PUBLIC_RPC!
const rpcUrl = 'https://soroban-testnet.stellar.org'

const keypair = Keypair.fromSecret('SAE27A5S6U32MAQBEB6GD4YAJFGGSSFINKB5QO64ZW32NBBMBYESNKN2')
const pubkey = keypair.publicKey()

let GLYPH: string | undefined = 'f158fdc6c03f09523b05e7f4026d72f9bc5cdb6a53a28cc3719895d275383f31'

let timeoutInSeconds = 30
let width: number = 16

const ColorglyphSDK = new Client({
    publicKey: pubkey,
    contractId,
    networkPassphrase,
    rpcUrl,
    async signTransaction(xdr: string) {
        const transaction = new Transaction(xdr, networkPassphrase)

        transaction.sign(keypair);

        return transaction.toXDR();
    }
});

const maxMineSize = 17
const maxMintSize = 18
const mineSanitizedPaletteArray: [number, number][][] = []
const mintSanitizedPaletteArray: [number, number[]][][] = []
const costs: string[] = []

let count = 0
let mineMap = new Map()
let mintMap = new Map()
let palette = JSON.parse(`[\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#0d47a1\",\"#0d47a1\",\"#0d47a1\",\"#000000\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#000000\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#000000\",\"#0d47a1\",\"#0d47a1\",\"#000000\",\"#ffd54f\",\"#ff80ab\",\"#e91e63\",\"#ff80ab\",\"#ff80ab\",\"#ff80ab\",\"#000000\",\"#b0bec5\",\"#000000\",\"#ff80ab\",\"#ff80ab\",\"#000000\",\"#b0bec5\",\"#000000\",\"#0d47a1\",\"#000000\",\"#ffd54f\",\"#ff80ab\",\"#ff80ab\",\"#ff80ab\",\"#ff80ab\",\"#e91e63\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#0d47a1\",\"#000000\",\"#ffd54f\",\"#ff80ab\",\"#ff80ab\",\"#e91e63\",\"#ff80ab\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#000000\",\"#ffd54f\",\"#e91e63\",\"#ff80ab\",\"#ff80ab\",\"#ff80ab\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#b0bec5\",\"#000000\",\"#000000\",\"#ffd54f\",\"#ff80ab\",\"#e91e63\",\"#ff80ab\",\"#ff80ab\",\"#000000\",\"#b0bec5\",\"#ff80ab\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#ff80ab\",\"#000000\",\"#000000\",\"#ffd54f\",\"#ff80ab\",\"#ff80ab\",\"#ff80ab\",\"#e91e63\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#b0bec5\",\"#000000\",\"#000000\",\"#000000\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#ffd54f\",\"#000000\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#b0bec5\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#e64a19\",\"#b0bec5\",\"#000000\",\"#000000\",\"#b0bec5\",\"#ffb74d\",\"#ffb74d\",\"#000000\",\"#b0bec5\",\"#000000\",\"#ffb74d\",\"#000000\",\"#b0bec5\",\"#000000\",\"#ffeb3b\",\"#ffeb3b\",\"#ffb74d\",\"#000000\",\"#000000\",\"#69f0ae\",\"#ffeb3b\",\"#ffeb3b\",\"#ffeb3b\",\"#000000\",\"#000000\",\"#000000\",\"#ffeb3b\",\"#000000\",\"#000000\",\"#000000\",\"#69f0ae\",\"#69f0ae\",\"#ffeb3b\",\"#2196f3\",\"#2196f3\",\"#2196f3\",\"#69f0ae\",\"#69f0ae\",\"#69f0ae\",\"#2196f3\",\"#2196f3\",\"#2196f3\",\"#69f0ae\",\"#69f0ae\",\"#69f0ae\",\"#2196f3\",\"#2196f3\",\"#2196f3\",\"#69f0ae\",\"#5c6bc0\",\"#5c6bc0\",\"#5c6bc0\",\"#2196f3\",\"#2196f3\",\"#2196f3\",\"#5c6bc0\",\"#5c6bc0\",\"#5c6bc0\",\"#2196f3\",\"#2196f3\",\"#2196f3\",\"#5c6bc0\",\"#5c6bc0\",\"#5c6bc0\",\"#2196f3\",\"#ab47bc\",\"#ab47bc\",\"#ab47bc\",\"#5c6bc0\",\"#5c6bc0\",\"#5c6bc0\",\"#ab47bc\",\"#ab47bc\",\"#ab47bc\",\"#5c6bc0\",\"#5c6bc0\",\"#5c6bc0\",\"#ab47bc\",\"#ab47bc\",\"#ab47bc\",\"#5c6bc0\"]`).map((hex: string) =>
    parseInt(hex.replace('#', ''), 16)
)

for (const i in palette) {
    const index = Number(i)
    const color = palette[index]
    const amount: number = mineMap.get(color) || 0
    
    mineMap.set(color, amount + 1)

    if (
        index === palette.length - 1
        || mineMap.size >= maxMineSize
    ) {
        mineSanitizedPaletteArray.push([...mineMap.entries()])
        mineMap = new Map()
    }
}

for (const i in palette) {
    const index = Number(i)
    const color = palette[index]
    const indexes: number[] = mintMap.get(color) || []

    indexes.push(index)
    mintMap.set(color, indexes)
    count++

    if (
        index === palette.length - 1
        || mintMap.size >= maxMintSize
    ) {
        mintSanitizedPaletteArray.push([...mintMap.entries()])
        count = 0
        mintMap = new Map()
    }
}

await super_mint();
// await glyph_get();

async function super_mint() {
    // console.log('mineables', mineSanitizedPaletteArray.length);

    // for (const mineMap of mineSanitizedPaletteArray) {
    //     const mineMap = new Map([...new Map(mineSanitizedPaletteArray[0]).entries()].sort((a, b) => a[0] - b[0]));

    //     let mineTx = await ColorglyphSDK.colors_mine(
    //         {
    //             source: pubkey,
    //             miner: undefined,
    //             to: undefined,
    //             colors: mineMap
    //         },
    //         { timeoutInSeconds }
    //     );

    //     let { getTransactionResponse: mineTxGetTransactionResponse } = await mineTx.signAndSend()

    //     if (mineTxGetTransactionResponse?.status === 'SUCCESS') {
    //         const cost = mineTxGetTransactionResponse.resultXdr.feeCharged().toString()
    //         costs.push(cost)
    //         console.log('mine', cost);
    //     } else {
    //         throw new Error('mine failed')
    //     }
    // }

    // return

    console.log('mintables', mintSanitizedPaletteArray.length);

    // for (const mintMap of mintSanitizedPaletteArray) {
        const mintMap = new Map()
        mintMap.set(pubkey, new Map([...new Map(mintSanitizedPaletteArray[0]).entries()].sort((a, b) => a[0] - b[0])))

        let mintTx = await ColorglyphSDK.glyph_mint(
            {
                minter: pubkey,
                to: undefined,
                colors: mintMap,
                width: 16
            },
            { timeoutInSeconds }
        );

        let { result, getTransactionResponse: mintTxGetTransactionResponse } = await mintTx.signAndSend()

        if (mintTxGetTransactionResponse?.status === 'SUCCESS') {
            const cost = mintTxGetTransactionResponse.resultXdr.feeCharged().toString()
            costs.push(cost)
            console.log('mint', cost);
        } else {
            throw new Error('mint failed')
        }
    // }

    // let tx = await ColorglyphSDK.glyph_mint(
    //     {
    //         minter: pubkey,
    //         to: undefined,
    //         colors: new Map(),
    //         width
    //     },
    //     { timeoutInSeconds }
    // );

    // let { result, getTransactionResponse } = await tx.signAndSend()

    // if (getTransactionResponse?.status === 'SUCCESS') {
    //     const cost = getTransactionResponse.resultXdr.feeCharged().toString()
    //     costs.push(cost)
    //     console.log(cost);
    // } else {
    //     throw new Error('final mint failed')
    // }

    const hash = result?.toString('hex');

    GLYPH = hash
    console.log(hash);

    glyph_get();
    console.log( 
        costs.reduce((a, b) => Number(a) + Number(b), 0),
    );
}

async function glyph_get() {
    let { result: res } = await ColorglyphSDK.glyph_get({
        hash_type: {
            tag: 'Glyph',
            values: [Buffer.from(GLYPH!, 'hex')]
        }
    })

    const glyph = res.unwrap().values[0] as Glyph;

    width = glyph.width

    palette = new Array(glyph.length).fill(256 ** 3 - 1);
    
    for (const [_account, colors] of glyph.colors) {
        for (const [color, indexes] of colors) {
            for (const index of indexes as number[]) {
                palette.splice(index, 1, Number(color))	
            }
        }
    }
}