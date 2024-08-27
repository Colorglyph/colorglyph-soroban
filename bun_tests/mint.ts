import { Client } from 'colorglyph-sdk'
import { Keypair, Networks, Transaction } from '@stellar/stellar-sdk'
import { getGlyphHash, paletteToBase64 } from './utils'

// const contractId = "CBDJNPHMQKKNBYJ5MQWGWF5XD2AECLQJRUCNQZ654XQD4SIFK6UCJPRY"
const contractId = "CARZSHD6BLSLB5ENFR76QI4VNJ2XUHXEDCRG77VMLOAICRG7MZTIZPA7"
// const networkPassphrase = Networks.PUBLIC
const networkPassphrase = Networks.TESTNET
// const rpcUrl = Bun.env.PUBLIC_RPC!
const rpcUrl = 'https://soroban-testnet.stellar.org'

// GBGP5SD75TDB2ZL7JDJEFPSWDBEQRDJ4757ZXL57TOOQJSMWROT5JYKD
const keypair = Keypair.fromSecret('SAE27A5S6U32MAQBEB6GD4YAJFGGSSFINKB5QO64ZW32NBBMBYESNKN2')
const pubkey = keypair.publicKey()

let GLYPH: string | undefined = '9eb925d1fe9970fc0e2e93ad1b4c8c1e92136600f9aac84b89dda44814d188cb';

let timeoutInSeconds = 30
let width = 16
let w = 1
let palette: number[] = []

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

let costs: string[] = []
await super_mint();
// await glyph_get();

async function super_mint() {
    let max_mine = 23;
    let max_mint = 23; // All but the first can actually be 24

    let colors = generateRGBSpectrum(width);
    let hash = await getGlyphHash(colors, w);
    let mintIndexes = new Map<number, number[]>();
    let mineColors = new Map(colors.map((color, index) => {
        mintIndexes.set(color, [index])
        return [color, 1]
    }));

    mineColors = new Map([...mineColors.entries()].sort((a, b) => a[0] - b[0]));
    mintIndexes = new Map([...mintIndexes.entries()].sort((a, b) => a[0] - b[0]));

    // for (let index = 0; index < Math.ceil(width ** 2 / max_mine); index++) {
    //     let map = Array.from(mineColors).slice(index * max_mine, index * max_mine + max_mine);
    //     let tx = await ColorglyphSDK.colors_mine(
    //         {
    //             source: pubkey,
    //             miner: undefined,
    //             to: undefined,
    //             colors: new Map(map)
    //         },
    //         { timeoutInSeconds }
    //     );

    //     let { getTransactionResponse } = await tx.signAndSend()

    //     if (getTransactionResponse?.status === 'SUCCESS') {
    //         const cost = getTransactionResponse.resultXdr.feeCharged().toString()
    //         costs.push(cost)
    //         console.log(cost);
    //     }

    //     console.log(index, 'mine');
    // }

    for (let index = 0; index < Math.ceil(width ** 2 / max_mint); index++) {
        let mintMap = new Map();
        let map = Array.from(mintIndexes).slice(index * max_mint, index * max_mint + max_mint);
        mintMap.set(pubkey, new Map(map));

        let tx = await ColorglyphSDK.glyph_mint(
            {
                hash,
                minter: pubkey,
                to: undefined,
                colors: mintMap,
                width: undefined
            },
            { timeoutInSeconds }
        );

        let { getTransactionResponse } = await tx.signAndSend()

        if (getTransactionResponse?.status === 'SUCCESS') {
            const cost = getTransactionResponse.resultXdr.feeCharged().toString()
            costs.push(cost)
            console.log(cost);
        }

        console.log(index, 'mint');
    }

    let tx = await ColorglyphSDK.glyph_mint(
        {
            hash,
            minter: pubkey,
            to: undefined,
            colors: new Map(),
            width: w
        },
        { timeoutInSeconds }
    );

    let { getTransactionResponse } = await tx.signAndSend()

    if (getTransactionResponse?.status === 'SUCCESS') {
        const cost = getTransactionResponse.resultXdr.feeCharged().toString()
        costs.push(cost)
        console.log(cost);
    }

    GLYPH = hash.toString('hex');
    console.log(GLYPH);

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

function generateRGBSpectrum(steps: number) {
    const colorArray = [];

    for (let i = 0; i < steps; i++) {
        for (let j = 0; j < steps; j++) {
            const red = 255 - Math.floor((i * 255) / steps);
            const green = 255 - Math.floor((j * 255) / steps);
            const blue = Math.floor((i * j * 255) / (steps * steps));

            const colorValue = red * Math.pow(256, 2) + green * 256 + blue;
            colorArray.push(colorValue);
        }
    }

    return colorArray;
}