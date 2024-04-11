import { $ } from 'bun'

const wasm_hash = (await $`soroban contract install --wasm ../target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --source keybase --network testnet`.text()).replace(/\W/g, '')
console.log('HASH', wasm_hash);

const salt = Number(Math.random().toString().substring(2)) // 8891
const contract_id = (await $`soroban contract deploy --wasm-hash ${wasm_hash} --source keybase --network testnet --salt ${salt}`.text()).replace(/\W/g, '')
console.log('CONTRACT', contract_id);

const token_address = (await $`soroban contract asset id --asset native --source keybase --network testnet`.text()).replace(/\W/g, '')
console.log('TOKEN', token_address);

await $`soroban contract invoke --id ${contract_id} --network testnet --source keybase -- initialize --owner-address keybase --token-address ${token_address} --fee_address keybase`
console.log('INITIALIZED');

const keybase = (await $`soroban keys address keybase`.text()).replace(/\W/g, '')
await $`soroban contract invoke --id ${contract_id} --network testnet --source keybase -- update --owner-address ${keybase} --token-address ${token_address} --fee-address ${keybase}`
console.log('UPDATED');

await $`soroban contract invoke --id ${contract_id} --network testnet --source keybase -- upgrade --hash ${wasm_hash}`
console.log('UPGRADED');

await $`soroban contract invoke --id ${contract_id} --network testnet --source keybase -- -h`