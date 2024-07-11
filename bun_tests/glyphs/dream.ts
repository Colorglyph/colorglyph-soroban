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
let palette = JSON.parse(`[\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#90a4ae\",\"#b0bec5\",\"#cfd8dc\",\"#eceff1\",\"#e0f2f1\",\"#b3e5fc\",\"#4a148c\",\"#6a1b9a\",\"#7b1fa2\",\"#8e24aa\",\"#9c27b0\",\"#ab47bc\",\"#ba68c8\",\"#ce93d8\",\"#e1bee7\",\"#f3e5f5\",\"#78909c\",\"#8bc34a\",\"#7cb342\",\"#689f38\",\"#e0f7fa\",\"#bbdefb\",\"#880e4f\",\"#ffb74d\",\"#ffcc80\",\"#ffe0b2\",\"#fff3e0\",\"#fbe9e7\",\"#ffccbc\",\"#ffab91\",\"#ff8a65\",\"#ede7f6\",\"#607d8b\",\"#9ccc65\",\"#c0ca33\",\"#558b2f\",\"#b2ebf2\",\"#82b1ff\",\"#880e4f\",\"#ffa726\",\"#8d6e63\",\"#827717\",\"#9e9d24\",\"#afb42b\",\"#c0ca33\",\"#cddc39\",\"#ff9800\",\"#d1c4e9\",\"#546e7a\",\"#aed581\",\"#afb42b\",\"#33691e\",\"#80deea\",\"#448aff\",\"#c2185b\",\"#ff9800\",\"#004d40\",\"#00695c\",\"#00796b\",\"#00897b\",\"#009688\",\"#d4e157\",\"#ff5722\",\"#b39ddb\",\"#455a64\",\"#c5e1a5\",\"#9e9d24\",\"#3e2723\",\"#4dd0e1\",\"#2979ff\",\"#d81b60\",\"#fb8c00\",\"#1b5e20\",\"#42a5f5\",\"#64b5f6\",\"#90caf9\",\"#4db6ac\",\"#d4e157\",\"#f4511e\",\"#9575cd\",\"#37474f\",\"#dcedc8\",\"#827717\",\"#4e342e\",\"#26c6da\",\"#2962ff\",\"#e91e63\",\"#f57c00\",\"#2e7d32\",\"#2196f3\",\"#e3f2fd\",\"#bbdefb\",\"#80cbc4\",\"#e6ee9c\",\"#e64a19\",\"#7e57c2\",\"#263238\",\"#f1f8e9\",\"#bcaaa4\",\"#5d4037\",\"#00bcd4\",\"#304ffe\",\"#ec407a\",\"#ef6c00\",\"#43a047\",\"#1e88e5\",\"#1976d2\",\"#1565c0\",\"#0d47a1\",\"#f0f4c3\",\"#fff8e1\",\"#673ab7\",\"#212121\",\"#b9f6ca\",\"#a1887f\",\"#6d4c41\",\"#00acc1\",\"#3d5afe\",\"#f06292\",\"#ff6f00\",\"#4caf50\",\"#66bb6a\",\"#81c784\",\"#a5d6a7\",\"#c8e6c9\",\"#e8f5e9\",\"#fff9c4\",\"#5e35b1\",\"#616161\",\"#69f0ae\",\"#8d6e63\",\"#795548\",\"#0097a7\",\"#536dfe\",\"#f48fb1\",\"#f57f17\",\"#f9a825\",\"#fbc02d\",\"#fdd835\",\"#ffeb3b\",\"#ffee58\",\"#fff176\",\"#fff59d\",\"#512da8\",\"#757575\",\"#00e676\",\"#00c853\",\"#006064\",\"#00838f\",\"#8c9eff\",\"#f8bbd0\",\"#fce4ec\",\"#ffebee\",\"#ffcdd2\",\"#ef9a9a\",\"#e57373\",\"#ef5350\",\"#f44336\",\"#e53935\",\"#4527a0\",\"#9e9e9e\",\"#bdbdbd\",\"#e0e0e0\",\"#ffffff\",\"#fafafa\",\"#e8eaf6\",\"#c5cae9\",\"#9fa8da\",\"#7986cb\",\"#5c6bc0\",\"#3f51b5\",\"#3949ab\",\"#303f9f\",\"#283593\",\"#1a237e\",\"#311b92\"]`).map((hex: string) =>
    parseInt(hex.replace('#', ''), 16)
)

console.log('unique', [...new Set(palette)].length);

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