use soroban_sdk::testutils::arbitrary::std::dbg;
use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal, String};

use crate::token::deploy_token_contract;

#[test]
fn test_to_show_dbg_info() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let token = deploy_token_contract(&env);
    token.initialize(&admin, &7, &"fuzzy".into_val(&env), &"FZY".into_val(&env));

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &500);
    assert_eq!(token.balance(&user2), 500);

    dbg!(token.name());
    dbg!(token.symbol());
    assert_eq!(token.symbol(), String::from_str(&env, "NOT"));
}
