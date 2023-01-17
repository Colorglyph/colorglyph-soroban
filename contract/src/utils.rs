use crate::{token::Client as TokenClient, types::{StorageKey, Error}};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{BytesN, Env, Address, panic_with_error};

pub fn get_token_bits(
    env: &Env,
    token_id: &BytesN<32>,
    signature: &Signature,
) -> (Identifier, Identifier, TokenClient, i128) {
    let contract_identifier = Identifier::Contract(env.get_current_contract());
    let signature_identifier = signature.identifier(env);
    let token = TokenClient::new(env, token_id);
    let sender_nonce = token.nonce(&signature_identifier);

    (
        contract_identifier,
        signature_identifier,
        token,
        sender_nonce,
    )
}

pub fn verify_glyph_ownership(env: &Env, glyph_hash: BytesN<32>) {
    let glyph_owner: Address = env
        .storage()
        .get(StorageKey::GlyphOwner(glyph_hash))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotAuthorized))
        .unwrap();

    if glyph_owner != env.invoker() {
        panic_with_error!(env, Error::NotAuthorized);
    }
}