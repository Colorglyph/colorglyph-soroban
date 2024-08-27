import { Client } from 'colorglyph-sdk'
import { Keypair, Networks, Transaction } from '@stellar/stellar-sdk'
import { getGlyphHash, paletteToBase64 } from '../utils'

// const contractId = "CAMCM4WTWRM2UZBI7CEQE4HI2PZEAE3ZBDDNJTI3RB6CIZRJKRGXEHWK"
const contractId = "CARZSHD6BLSLB5ENFR76QI4VNJ2XUHXEDCRG77VMLOAICRG7MZTIZPA7"
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

const maxMineSize = 23
const maxMintSize = 23
const mineSanitizedPaletteArray: [number, number][][] = []
const mintSanitizedPaletteArray: [number, number[]][][] = []
const costs: string[] = []

let count = 0
let mineMap = new Map()
let mintMap = new Map()
let palette = new Array(256).fill('#0000ff').map((hex: string) =>
    parseInt(hex.replace('#', ''), 16)
)
let hash = await getGlyphHash(palette, width);

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
    for (const mineMap of mineSanitizedPaletteArray) {
        const map = new Map([...new Map(mineMap).entries()].sort((a, b) => a[0] - b[0]));

        let tx = await ColorglyphSDK.colors_mine(
            {
                source: pubkey,
                miner: undefined,
                to: undefined,
                colors: map
            },
            { timeoutInSeconds }
        );

        let { getTransactionResponse } = await tx.signAndSend()

        if (getTransactionResponse?.status === 'SUCCESS') {
            const cost = getTransactionResponse.resultXdr.feeCharged().toString()
            costs.push(cost)
            console.log('mine', cost);
        } else {
            throw new Error('mine failed')
        }
    }

    for (const mintMap of mintSanitizedPaletteArray) {
        const map = new Map()
        map.set(pubkey, new Map([...new Map(mintMap).entries()].sort((a, b) => a[0] - b[0])))

        let tx = await ColorglyphSDK.glyph_mint(
            {
                hash,
                minter: pubkey,
                to: undefined,
                colors: map,
                width: undefined
            },
            { timeoutInSeconds }
        );

        let { getTransactionResponse } = await tx.signAndSend()

        if (getTransactionResponse?.status === 'SUCCESS') {
            const cost = getTransactionResponse.resultXdr.feeCharged().toString()
            costs.push(cost)
            console.log('mint', cost);
        } else {
            throw new Error('mint failed')
        }
    }

    let tx = await ColorglyphSDK.glyph_mint(
        {
            hash,
            minter: pubkey,
            to: undefined,
            colors: new Map(),
            width
        },
        { timeoutInSeconds }
    );

    let { getTransactionResponse } = await tx.signAndSend()

    if (getTransactionResponse?.status === 'SUCCESS') {
        const cost = getTransactionResponse.resultXdr.feeCharged().toString()
        costs.push(cost)
        console.log(cost);
    } else {
        throw new Error('final mint failed')
    }

    GLYPH = hash.toString('hex');
    console.log(hash);

    glyph_get();

    console.log( 
        costs.reduce((a, b) => Number(a) + Number(b), 0),
    );
}

async function glyph_get() {
    let { result: res } = await ColorglyphSDK.glyph_get({
        hash: Buffer.from(GLYPH!, 'hex')
    })

    const glyph = res.unwrap();

    width = glyph.width

    palette = new Array(glyph.length).fill(256 ** 3 - 1);
    
    for (const [_account, colors] of glyph.colors) {
        for (const [color, indexes] of colors) {
            for (const index of indexes as number[]) {
                palette.splice(index, 1, Number(color))	
            }
        }
    }

    console.log(
        Buffer.from(await paletteToBase64(palette, width)).toString('base64')
    );
}