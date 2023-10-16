# alias soroban="(cd ~/Desktop/Web/Soroban/soroban-tools/; cargo build --quiet) && ~/Desktop/Web/Soroban/soroban-tools/target/debug/soroban"

# make clean
make build-opt

contract_id=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --source default --network local)
echo 'contract deployed' $contract_id
# contract_id=CBHW2VG2H6Q5TCDIAM37KX5FZGZUDSWBXIYMWNG3W2FDSYB7UTIO44NS

token_address=CDMLFMKMMD7MWZP3FKUBZPVHTUEDLSX4BYGYKH4GCESXYHS3IHQ4EIG4

fee_address=GA55USY2TY4DEO5YFQ3KZECL2A3A5IVYVCKPB4LLTAE57TOE6PM46D7C
# SBSBEB2WAVVRO3ITSLJACCQNSL67KIBV46FYGELSZXNGJCKYN5KL3F7P

# initialize
soroban contract invoke --id $contract_id --source default --network local -- initialize --token_address $token_address --fee_address $fee_address
echo 'contract initialized'

# soroban contract bindings typescript --wasm target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --contract-id $contract_id --output-dir ./colorglyph-sdk --overwrite
# echo 'bindings generated'