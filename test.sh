make

contract_id=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/colorglyph.wasm --source Default --network Futurenet)
echo 'contract deployed' $contract_id
# contract_id="cb90acf8e85ae452f1057af2f09149bc4ec6e5fe8e7099ab0bd06fd8be9607d3"

# d93f5c7bb0ebc4a9c8f727c5cebc4e41194d38257e1d0d910356b43bfc528813
token_id=CDMT6XD3WDV4JKOI64T4LTV4JZARSTJYEV7B2DMRANLLIO74KKEBHYNJ

fee_address=GBOSPZUIXJSFGUGMFPGR4KTOCR6VDZRUBXSCZW34GP5MERXLPWMJZX7J

user1_pk=GC5HBZMF4QX475QYOJ7XB5VZZZH3DP6N6X2ENJBD7OY7VTZ265YLHAZR
user1_sk=SBQOVS7GB4XL3XDFS5IB6OJDQBJ46U3JBX4XU36OUTGOFNMMKJ3SXQPW

user2_pk=GCV7OMN4YBEP7RQTKZNE5UP22QKGFXUYMJMSNY564CKODR3HGLY5KJXE
user2_sk=SDF6OA6FWCEDDRL5NGUDMG6K7ZTEVQCLF24TKPBWALXLCD42IJZ7I5A5

# initialize
soroban contract invoke --id $contract_id --source Default --network Futurenet -- initialize --token_id $token_id --fee_address $fee_address
echo 'contract initialized'

# colors_mine
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- colors_mine --miner $user1_pk --colors '[[0, 100], [16777215, 100]]'
echo 'colors mined'

# colors_transfer
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- colors_transfer --from $user1_pk --to $user2_pk --colors '[["'$user1_pk'", 0, 1]]'
echo 'colors transferred'

# color_balance
soroban contract invoke --id $contract_id --source Default --network Futurenet -- color_balance --owner $user1_pk --color 0

# glyph_mint
glyph_hash=$(soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- glyph_mint --minter $user1_pk --colors '[["'$user1_pk'", [[0, [0, 2]], [16777215, [1, 3]]]]]' --width 2 | tr -d '"')
echo 'glyph minted' $glyph_hash
# glyph_hash="8c00b09eab0b569cbf72385cb5c5ba428530cedc701add1677ed0b1a321d82cc"

# [
#     [
#         "'$user1_pk'",
#         [
#             [0, [0, 2]], [16777215, [1, 3]]
#         ]
#     ]
# ]

# glyph_get
soroban contract invoke --id $contract_id --source Default --network Futurenet -- glyph_get --hash $glyph_hash

# offer_post
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- offer_post --seller $user1_pk --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_id'", "100"]}'
echo 'offer posted'

# offers_get
soroban contract invoke --id $contract_id --source Default --network Futurenet -- offers_get --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_id'", "100"]}'

# offer_delete
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- offer_delete --seller $user1_pk --sell '{"Glyph": "'$glyph_hash'"}' --buy '{"Asset": ["'$token_id'", "100"]}'
echo 'offer deleted'

# glyph_scrape
soroban contract invoke --id $contract_id --source $user1_sk --network Futurenet -- glyph_scrape --owner $user1_pk --hash $glyph_hash
echo 'glyph scraped'