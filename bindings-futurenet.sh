# alias soroban="(cd ~/Desktop/Web/Soroban/soroban-tools/; cargo build --quiet) && ~/Desktop/Web/Soroban/soroban-tools/target/debug/soroban"

# make clean
# make build-opt

contract_id=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --source default --network futurenet --fee 10000000)
echo 'contract deployed' $contract_id
# contract_id=CC5YDEXJXBOBNUSHX52DIIAOOJJMACKFTFU7FRIWJRRNZG2K5YRCETVW

token_address=CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT

fee_address=GA55USY2TY4DEO5YFQ3KZECL2A3A5IVYVCKPB4LLTAE57TOE6PM46D7C
# SBSBEB2WAVVRO3ITSLJACCQNSL67KIBV46FYGELSZXNGJCKYN5KL3F7P

# initialize
soroban contract invoke --id $contract_id --source default --network futurenet -- initialize --token_address $token_address --fee_address $fee_address
echo 'contract initialized'

soroban contract bindings typescript --wasm target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --contract-id $contract_id --network futurenet --output-dir ./colorglyph-sdk --overwrite
echo 'bindings generated'