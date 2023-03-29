use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

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

#[test]
fn integration_test() {
    println!("Running integration test...");
    let mut router = mock_app();

    let child_codeid = router.store_code(child_contract());

    let factory_codeid = router.store_code(factory_contract());
    let factory_owner = "OWNER";
    let factory_contract_addr = router
        .instantiate_contract(
            factory_codeid,
            Addr::unchecked(factory_owner),
            &factory::msg::InstantiateMsg {
                child_codeid: child_codeid,
            },
            &[],                // funds
            "Contract Factory", // label
            None,               // code admin (for migration)
        )
        .unwrap();

    let execute_msg = factory::msg::ExecuteMsg::NewContract {};
    let res = router
        .execute_contract(
            Addr::unchecked(factory_owner),
            factory_contract_addr.clone(),
            &execute_msg,
            &[], // funds
        )
        .unwrap();
    let instantiation_event = res.events[res.events.len() - 1].clone();
    println!(
        "Instantiated code id {} to address {}",
        instantiation_event.attributes[1].value, instantiation_event.attributes[0].value
    );

    let query_msg = factory::msg::QueryMsg::Children {};
    let query_response: factory::msg::ChildrenResponse = router
        .wrap()
        .query_wasm_smart(factory_contract_addr, &query_msg)
        .unwrap();
    assert_eq!(query_response.children.len(), 1);
    assert_eq!(query_response.children[0], "contract1".to_string());
}
