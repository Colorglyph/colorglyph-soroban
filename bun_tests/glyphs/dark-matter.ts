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

let GLYPH: string | undefined // = 'd6e620401a48aabed49a61779c4b2bea6d4253078444744115637aa2a225fe75'

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
let palette = JSON.parse(`[\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#e1bee7\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#7e57c2\",\"#b39ddb\",\"#f3e5f5\",\"#d1c4e9\",\"#b39ddb\",\"#d1c4e9\",\"#ffffff\",\"#ffffff\",\"#9575cd\",\"#b39ddb\",\"#d1c4e9\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#b39ddb\",\"#673ab7\",\"#7e57c2\",\"#9575cd\",\"#9575cd\",\"#9575cd\",\"#b39ddb\",\"#311b92\",\"#311b92\",\"#5e35b1\",\"#b39ddb\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#7e57c2\",\"#673ab7\",\"#3949ab\",\"#5e35b1\",\"#5e35b1\",\"#311b92\",\"#b39ddb\",\"#ffffff\",\"#311b92\",\"#9575cd\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#f3e5f5\",\"#b39ddb\",\"#9575cd\",\"#3949ab\",\"#311b92\",\"#311b92\",\"#311b92\",\"#5e35b1\",\"#7e57c2\",\"#311b92\",\"#d1c4e9\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#7e57c2\",\"#b388ff\",\"#b388ff\",\"#5e35b1\",\"#5e35b1\",\"#1a237e\",\"#000000\",\"#000000\",\"#000000\",\"#311b92\",\"#5e35b1\",\"#7e57c2\",\"#b39ddb\",\"#f3e5f5\",\"#ffffff\",\"#7e57c2\",\"#673ab7\",\"#ffffff\",\"#d1c4e9\",\"#5e35b1\",\"#1a237e\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#000000\",\"#3949ab\",\"#5e35b1\",\"#9575cd\",\"#673ab7\",\"#d1c4e9\",\"#7e57c2\",\"#673ab7\",\"#d1c4e9\",\"#5e35b1\",\"#311b92\",\"#1a237e\",\"#000000\",\"#000000\",\"#aa00ff\",\"#000000\",\"#000000\",\"#1a237e\",\"#3949ab\",\"#673ab7\",\"#d1c4e9\",\"#ffffff\",\"#ffffff\",\"#7e57c2\",\"#673ab7\",\"#673ab7\",\"#5e35b1\",\"#5e35b1\",\"#1a237e\",\"#000000\",\"#000000\",\"#000000\",\"#1a237e\",\"#7e57c2\",\"#283593\",\"#7e57c2\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#f3e5f5\",\"#d1c4e9\",\"#b39ddb\",\"#7e57c2\",\"#673ab7\",\"#1a237e\",\"#000000\",\"#1a237e\",\"#3949ab\",\"#9575cd\",\"#7e57c2\",\"#673ab7\",\"#f3e5f5\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#f3e5f5\",\"#7e57c2\",\"#673ab7\",\"#7e57c2\",\"#673ab7\",\"#1a237e\",\"#673ab7\",\"#673ab7\",\"#b39ddb\",\"#d1c4e9\",\"#f3e5f5\",\"#673ab7\",\"#d1c4e9\",\"#ffffff\",\"#ffffff\",\"#f3e5f5\",\"#7e57c2\",\"#d1c4e9\",\"#7e57c2\",\"#9575cd\",\"#7e57c2\",\"#673ab7\",\"#283593\",\"#673ab7\",\"#b388ff\",\"#ffffff\",\"#ffffff\",\"#d1c4e9\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#f3e5f5\",\"#ffffff\",\"#7e57c2\",\"#d1c4e9\",\"#283593\",\"#b39ddb\",\"#d1c4e9\",\"#7e57c2\",\"#f3e5f5\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#7e57c2\",\"#f3e5f5\",\"#d1c4e9\",\"#673ab7\",\"#9575cd\",\"#ffffff\",\"#ffffff\",\"#7e57c2\",\"#b388ff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#f3e5f5\",\"#ffffff\",\"#f3e5f5\",\"#7e57c2\",\"#673ab7\",\"#d1c4e9\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#f3e5f5\",\"#d1c4e9\",\"#7e57c2\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\",\"#ffffff\"]`).map((hex: string) =>
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