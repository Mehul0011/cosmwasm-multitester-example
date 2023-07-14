use cosmwasm_std::{Addr, Empty};
use cosmwasm_std::{coin, Coin, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

// put common functions here, such as ContractWrapper functions

#[allow(dead_code)]
fn mock_app() -> App {
    App::default()
}

#[allow(dead_code)]
fn child_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        child::contract::execute,
        child::contract::instantiate,
        child::contract::query,
    )
    // .with_reply(child::contract::reply);
    .with_migrate(child::contract::migrate);
    Box::new(contract)
}

#[allow(dead_code)]
fn factory_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        factory::contract::execute,
        factory::contract::instantiate,
        factory::contract::query,
    )
    .with_reply(factory::contract::reply);
    Box::new(contract)
}

pub fn mint_native(app: &mut App, recipient: String, denom: String, amount: u128) {
    app.sudo(cw_multi_test::SudoMsg::Bank(
        cw_multi_test::BankSudo::Mint {
            to_address: recipient,
            amount: vec![coin(amount, denom)],
        },
    ))
    .unwrap();
}

#[test]
fn integration_test() {
    // set up mock app
    let mut router = mock_app();
    // set up contracts
    let child_codeid = app.store_code(child_contract());
    
    let factory_codeid = router.store_code(factory_contract());
    // instantiate factory

    let factory_contract_addr = router
        .instantiate_contract(
            factory_codeid,
            Addr::unchecked(factory_owner),
            &factory::msg::InstantiateMsg { child_codeid },
            &[],                   // funds
            "Contract Factory",    // label
            None,                  // code admin (for migration)
        )
        .unwrap();

    // execute NewAccount so factory instantiates child
    let execute_msg = factory::msg::ExecuteMsg::NewContract {};
    let res = router
        .execute_contract(
            Addr::unchecked(factory_owner),
            factory_contract_addr.clone(),    // clone since we'll use it again
            &execute_msg,
            &[],                              // funds
        )
    .unwrap();

    let instantiation_event = res.events[res.events.len() - 1].clone();
    println!("Instantiated code id {} to address {}",
        instantiation_event.attributes[1].value,
        instantiation_event.attributes[0].value
    );
    // query factory to confirm it has child address stored
    let query_msg = factory::msg::QueryMsg::Children {};
    let query_response: factory::msg::ChildrenResponse = router
        .wrap()
        .query_wasm_smart(
            factory_contract_addr,
            &query_msg,
        )
        .unwrap();
    assert_eq!(query_response.children.len(), 1);
    assert_eq!(query_response.children[0], "contract1".to_string());

    let old_block_info = router.block_info();
    router.set_block(BlockInfo {
        height: 12345u64,
        time: Timestamp::from_seconds(old_block_info.time.seconds() + 74056),
        chain_id: old_block_info.chain_id,
    });


    mint_native(
        &mut router,
        "alice".to_string(),
        "umulti".to_string(),
        1_100_000u128,
    );

    router
    .send_tokens(
        "alice".to_string(),  // sender
        "bob".to_string(),    // recipient
        &[coin(500_000u128, "umulti"],
    )
    .unwrap();
}
