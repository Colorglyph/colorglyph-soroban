make

contract_id=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/colorglyph.wasm --source default --network futurenet)
echo 'contract deployed' $contract_id
# contract_id=CDZQMN3LMV5CGCHYQM7PQZ3SJ5PPI7B5LSEGGYSIDT6VUQ2VB56F5CSX

token_address=CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT

fee_address=GA55USY2TY4DEO5YFQ3KZECL2A3A5IVYVCKPB4LLTAE57TOE6PM46D7C

user1_pk=GC5HBZMF4QX475QYOJ7XB5VZZZH3DP6N6X2ENJBD7OY7VTZ265YLHAZR
user1_sk=SBQOVS7GB4XL3XDFS5IB6OJDQBJ46U3JBX4XU36OUTGOFNMMKJ3SXQPW

user2_pk=GCV7OMN4YBEP7RQTKZNE5UP22QKGFXUYMJMSNY564CKODR3HGLY5KJXE
user2_sk=SDF6OA6FWCEDDRL5NGUDMG6K7ZTEVQCLF24TKPBWALXLCD42IJZ7I5A5

# initialize
soroban contract invoke --id $contract_id --source default --network futurenet -- initialize --token_address $token_address --fee_address $fee_address
echo 'contract initialized'

# colors_mine
soroban contract invoke --id $contract_id --source $user1_sk --network futurenet --fee 1000000000 -- colors_mine --miner $user1_pk --colors '{"0": 100, "16777215": 100}'
echo 'colors mined'

# glyph_mint (partial)
glyph_hash=$(soroban contract invoke --id $contract_id --source $user1_sk --network futurenet --fee 1000000000 -- glyph_mint --minter $user1_pk --colors '{"'$user1_pk'": {"0": [0, 1], "16777215": [2, 3]}}')
echo 'glyph minted partially'

# colors_transfer
soroban contract invoke --id $contract_id --source $user1_sk --network futurenet -- colors_transfer --from $user1_pk --to $user2_pk --colors '[["'$user1_pk'", 0, 1]]'
echo 'colors transferred'

# color_balance
soroban contract invoke --id $contract_id --source default --network futurenet -- color_balance --owner $user2_pk --miner $user1_pk --color 0

glyph_mint
glyph_hash=$(soroban contract invoke --id $contract_id --source $user1_sk --network futurenet -- glyph_mint --minter $user1_pk --colors '{}' --width 2 | tr -d '"')
echo 'glyph minted' $glyph_hash
# glyph_hash="f0bc10a894428074df05e22065498100da657071704cd89ba4f1e3fb4b13f928"

# glyph_get
soroban contract invoke --id $contract_id --source default --network futurenet -- glyph_get --hash_type '{"Glyph": "'$glyph_hash'"}'

# glyph_transfer
soroban contract invoke --id $contract_id --source $user1_sk --network futurenet --fee 1000000000 -- glyph_transfer --to $user2_pk --hash_type '{"Glyph": "'$glyph_hash'"}'

# offer_post
soroban contract invoke --id $contract_id --source $user2_sk --network futurenet -- offer_post --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_address'", "100"]}'
echo 'offer posted'

# offers_get
soroban contract invoke --id $contract_id --source default --network futurenet --fee 1000000000 -- offers_get --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_address'", "100"]}'

# offer_delete
soroban contract invoke --id $contract_id --source $user2_sk --network futurenet -- offer_delete --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_address'", "100"]}'
echo 'offer deleted'

# glyph_scrape
soroban contract invoke --id $contract_id --source $user2_sk --network futurenet -- glyph_scrape --hash_type '{"Glyph": "'$glyph_hash'"}'
echo 'glyph scraped'