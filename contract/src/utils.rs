use crate::token::Client as TokenClient;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{BytesN, Env};

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
