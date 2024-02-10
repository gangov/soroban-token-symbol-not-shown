use soroban_sdk::Env;

mod token_wasm {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
}

pub fn deploy_token_contract<'a>(env: &Env) -> token_wasm::Client<'a> {
    token_wasm::Client::new(env, &env.register_contract_wasm(None, token_wasm::WASM))
}
