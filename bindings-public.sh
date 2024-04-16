contract_id=CAMCM4WTWRM2UZBI7CEQE4HI2PZEAE3ZBDDNJTI3RB6CIZRJKRGXEHWK

# token_address=CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA

# fee_address=GBDVX4VELCDSQ54KQJYTNHXAHFLBCA77ZY2USQBM4CSHTTV7DME7KALE

# initialize
# soroban contract invoke --id $contract_id --source default --network vc -- initialize --token_address $token_address --fee_address $fee_address --owner_address default
# echo 'contract initialized'

soroban contract bindings typescript --wasm target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm --contract-id $contract_id --network vc --output-dir ./colorglyph-sdk --overwrite
echo 'bindings generated'