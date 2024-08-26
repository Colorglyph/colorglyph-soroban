import { encodePng } from '@lunapaint/png-codec'

export async function paletteToBase64(palette: number[], width: number) {
    const rgb_palette: number[] = []

    for (const color of palette) {
        rgb_palette.push(...[
            color >> 16,
            color >> 8 & 0xff,
            color & 0xff,
            255
        ])
    }

    const { data } = await encodePng({
        data: new Uint8Array(rgb_palette),
        width: width,
        height: Math.ceil(palette.length / width), 
    })
    
    return data
}

export async function getGlyphHash(palette: number[], width: number) {
    const rgb_palette: number[] = []

    for (const color of palette) {
        rgb_palette.push(...[
            color >> 16,
            color >> 8 & 0xff,
            color & 0xff
        ])
    }

    rgb_palette.push(width)

    const digest = new Uint8Array(
        await crypto.subtle.digest(
            { name: 'SHA-256' },
            new Uint8Array(rgb_palette)
        )
    )

    return Buffer.from(digest)
}