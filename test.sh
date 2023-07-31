make

contract_id=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/colorglyph.wasm --source Default --network Futurenet)
echo 'contract deployed' $contract_id
# contract_id=CAMSMKKWJA2KOSYLY676V2YDXKK5P6LWI52MFREBGBLXVUXXXP7JHJKE

# 7dc1ecdf9335199fc9918dbe0c732ce1d1146aa8f29cc9c360afc6a747ae94df
token_address=CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT

fee_address=GBOSPZUIXJSFGUGMFPGR4KTOCR6VDZRUBXSCZW34GP5MERXLPWMJZX7J

user1_pk=GC5HBZMF4QX475QYOJ7XB5VZZZH3DP6N6X2ENJBD7OY7VTZ265YLHAZR
user1_sk=SBQOVS7GB4XL3XDFS5IB6OJDQBJ46U3JBX4XU36OUTGOFNMMKJ3SXQPW

user2_pk=GCV7OMN4YBEP7RQTKZNE5UP22QKGFXUYMJMSNY564CKODR3HGLY5KJXE
user2_sk=SDF6OA6FWCEDDRL5NGUDMG6K7ZTEVQCLF24TKPBWALXLCD42IJZ7I5A5

# initialize
soroban contract invoke --id $contract_id --source Default --network Futurenet -- initialize --token_address $token_address --fee_address $fee_address
echo 'contract initialized'

# colors_mine
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet --fee 1000000000 -- colors_mine --miner $user1_pk --colors '{"0": 100, "16777215": 100}'
echo 'colors mined'

# glyph_mint (partial)
glyph_hash=$(soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet --fee 1000000000 -- glyph_mint --minter $user1_pk --colors '{"'$user1_pk'": {"0": [0, 1]}}')
echo 'glyph minted partially'

# colors_transfer
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- colors_transfer --from $user1_pk --to $user2_pk --colors '[["'$user1_pk'", 0, 1]]'
echo 'colors transferred'

# color_balance
soroban contract invoke --id $contract_id --source Default --network Futurenet -- color_balance --owner $user1_pk --color 0

# glyph_mint
glyph_hash=$(soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- glyph_mint --minter $user1_pk --colors '{"'$user1_pk'": {"16777215": [2, 3]}}' --width 2 | tr -d '"')
echo 'glyph minted' $glyph_hash
# glyph_hash="74bf48c8c20e4f6fa3a94d548400de5832ee8c5daef3fc0c7eefc909b0142f20"

# glyph_get
soroban contract invoke --id $contract_id --source Default --network Futurenet -- glyph_get --hash_type '{"Glyph": "'$glyph_hash'"}'

# glyph_transfer
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet --fee 1000000000 -- glyph_transfer --to $user2_pk --hash_type '{"Glyph": "'$glyph_hash'"}'

# offer_post
soroban contract invoke --id $contract_id --source $user2_sk --network Futurenet -- offer_post --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_address'", "100"]}'
echo 'offer posted'

# offers_get
soroban contract invoke --id $contract_id --source Default --network Futurenet --fee 1000000000 -- offers_get --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_address'", "100"]}'

# offer_delete
soroban contract invoke --id $contract_id --source $user2_sk --network Futurenet -- offer_delete --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_address'", "100"]}'
echo 'offer deleted'

# glyph_scrape
soroban contract invoke --id $contract_id --source $user2_sk --network Futurenet -- glyph_scrape --owner $user2_pk --hash_type '{"Glyph": "'$glyph_hash'"}'
echo 'glyph scraped'