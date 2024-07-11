import { Client } from 'colorglyph-sdk'
import type { Glyph } from 'colorglyph-sdk'
import { Keypair, Networks, Transaction } from '@stellar/stellar-sdk'

// const contractId = "CAMCM4WTWRM2UZBI7CEQE4HI2PZEAE3ZBDDNJTI3RB6CIZRJKRGXEHWK"
const contractId = "CAUEYBG456425X627TP7JGLZTJOGYSH3XBDKNBTPUXOFIVVYYQ3UTHFR"
// const networkPassphrase = Networks.PUBLIC
const networkPassphrase = Networks.TESTNET
// const rpcUrl = Bun.env.PUBLIC_RPC!
const rpcUrl = 'https://soroban-testnet.stellar.org'

const keypair = Keypair.fromSecret('SAE27A5S6U32MAQBEB6GD4YAJFGGSSFINKB5QO64ZW32NBBMBYESNKN2')
const pubkey = keypair.publicKey()

let GLYPH: string | undefined = '5347af410646167f8b5b1514f6db164d6138a3709e4f583777dc8da22079cd66'

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
let palette = ["#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#00ff00", "#ffffff", "#00ff00", "#ffccbc", "#00ff00", "#00ff00", "#00ff00", "#ffccbc", "#c0ca33", "#c0ca33", "#c0ca33", "#c0ca33", "#00ff00", "#00ff00", "#00ff00", "#ffccbc", "#ffffff", "#00ff00", "#4e342e", "#ffccbc", "#ffccbc", "#00ff00", "#ffccbc", "#c0ca33", "#689f38", "#689f38", "#689f38", "#689f38", "#ffccbc", "#00ff00", "#ffccbc", "#ffccbc", "#ffccbc", "#4e342e", "#ffccbc", "#4e342e", "#ffffff", "#ffccbc", "#c0ca33", "#c0ca33", "#ffccbc", "#ffccbc", "#c0ca33", "#ffccbc", "#ffccbc", "#ffccbc", "#ffccbc", "#ffccbc", "#ffffff", "#ffccbc", "#4e342e", "#ffccbc", "#4e342e", "#c0ca33", "#c0ca33", "#000000", "#ffe0b2", "#000000", "#c0ca33", "#ffccbc", "#ffccbc", "#ffccbc", "#ffccbc", "#ffccbc", "#ffccbc", "#4e342e", "#ffccbc", "#4e342e", "#c0ca33", "#c0ca33", "#c0ca33", "#ffe0b2", "#ffe0b2", "#ffe0b2", "#c0ca33", "#c0ca33", "#ffccbc", "#ffccbc", "#ffccbc", "#ffab91", "#ffab91", "#ffccbc", "#4e342e", "#4e342e", "#689f38", "#c0ca33", "#c0ca33", "#ffe0b2", "#ffe0b2", "#ffe0b2", "#689f38", "#c0ca33", "#8d6e63", "#ffccbc", "#ffab91", "#ffab91", "#ffab91", "#ffab91", "#ffab91", "#8d6e63", "#8d6e63", "#c0ca33", "#c0ca33", "#4e342e", "#4e342e", "#4e342e", "#8d6e63", "#689f38", "#c0ca33", "#8d6e63", "#ffab91", "#ffab91", "#ffab91", "#ffab91", "#8d6e63", "#8d6e63", "#8d6e63", "#8d6e63", "#c0ca33", "#ffccbc", "#ffccbc", "#6d4c41", "#4e342e", "#8d6e63", "#689f38", "#ffab91", "#ffab91", "#ffab91", "#ffab91", "#ffab91", "#d4e157", "#ffe0b2", "#8d6e63", "#8d6e63", "#689f38", "#6d4c41", "#6d4c41", "#6d4c41", "#6d4c41", "#4e342e", "#ffe0b2", "#ffe0b2", "#ffab91", "#ff6e40", "#ff6e40", "#ffab91", "#ffe0b2", "#ffe0b2", "#ffccbc", "#6d4c41", "#689f38", "#6d4c41", "#8d6e63", "#6d4c41", "#8d6e63", "#4e342e", "#ffe0b2", "#ffe0b2", "#ff6e40", "#ff6e40", "#ff6e40", "#ffe0b2", "#ffe0b2", "#ffccbc", "#d4e157", "#4e342e", "#6d4c41", "#6d4c41", "#6d4c41", "#8d6e63", "#6d4c41", "#4e342e", "#ffccbc", "#ffe0b2", "#ffe0b2", "#ff6e40", "#ff6e40", "#ffe0b2", "#ffe0b2", "#d4e157", "#d4e157", "#c0ca33", "#4e342e", "#6d4c41", "#8d6e63", "#6d4c41", "#8d6e63", "#4e342e", "#ff6e40", "#ffccbc", "#ffe0b2", "#ff6e40", "#ff6e40", "#ffe0b2", "#ffe0b2", "#d4e157", "#d4e157", "#c0ca33", "#4e342e", "#8d6e63", "#6d4c41", "#8d6e63", "#6d4c41", "#4e342e", "#ff6e40", "#ffccbc", "#ffe0b2", "#ff6e40"].map((hex) =>
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
            minter: pubkey,
            to: undefined,
            colors: new Map(),
            width
        },
        { timeoutInSeconds }
    );

    let { result, getTransactionResponse } = await tx.signAndSend()

    if (getTransactionResponse?.status === 'SUCCESS') {
        const cost = getTransactionResponse.resultXdr.feeCharged().toString()
        costs.push(cost)
        console.log(cost);
    } else {
        throw new Error('final mint failed')
    }

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