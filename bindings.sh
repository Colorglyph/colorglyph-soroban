# alias soroban="(cd ~/Desktop/Web/Soroban/soroban-tools/; cargo build --quiet) && ~/Desktop/Web/Soroban/soroban-tools/target/debug/soroban"

make clean
make build-opt

contract_id=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --source default --network futurenet)
echo 'contract deployed' $contract_id
# contract_id=CAMSMKKWJA2KOSYLY676V2YDXKK5P6LWI52MFREBGBLXVUXXXP7JHJKE

# 7dc1ecdf9335199fc9918dbe0c732ce1d1146aa8f29cc9c360afc6a747ae94df
token_address=CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT

# SBSBEB2WAVVRO3ITSLJACCQNSL67KIBV46FYGELSZXNGJCKYN5KL3F7P
fee_address=GA55USY2TY4DEO5YFQ3KZECL2A3A5IVYVCKPB4LLTAE57TOE6PM46D7C

# initialize
soroban contract invoke --id $contract_id --source default --network futurenet -- initialize --token_address $token_address --fee_address $fee_address
echo 'contract initialized'

soroban contract bindings typescript --wasm target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --contract-id $contract_id --output-dir ./colorglyph-sdk
echo 'bindings generated'