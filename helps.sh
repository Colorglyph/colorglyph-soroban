GBYRWFISH63ZGXSYJGHWCH3CDHVDJY4ONGIC7H5JG6LCEDHM3XD6DA7S
SCF3AGOMV35HCVX7KQEJOG6MJ7GXMUPTUCKUKZR6HFZNBANNKJJUGW3B

soroban deploy \
--wasm target/wasm32-unknown-unknown/release/soroban_colors_contract.wasm \
--secret-key SCF3AGOMV35HCVX7KQEJOG6MJ7GXMUPTUCKUKZR6HFZNBANNKJJUGW3B \
--rpc-url https://kalepail-futurenet.stellar.quest:443/soroban/rpc \
--network-passphrase "Test SDF Future Network ; October 2022"

87023d13dd686c22c72be971ac4d3c78e822eacba8accb9dbf313144ba8c4726

soroban invoke \
--secret-key SCF3AGOMV35HCVX7KQEJOG6MJ7GXMUPTUCKUKZR6HFZNBANNKJJUGW3B \
--rpc-url https://kalepail-futurenet.stellar.quest:443/soroban/rpc \
--network-passphrase "Test SDF Future Network ; October 2022" \
--id 87023d13dd686c22c72be971ac4d3c78e822eacba8accb9dbf313144ba8c4726 \
--fn mine \
--arg '{"0":1}' \
--arg '{"object": {"vec": [{"symbol": "AccountId"}, {"object":{"account_id":{"public_key_type_ed25519":"711b15123fb7935e58498f611f6219ea34e38e69902f9fa93796220cecddc7e1"}}}]}}'