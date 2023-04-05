use cosmwasm_std::{to_binary, Deps, OwnedDeps, QueryRequest, StdResult, Uint128, WasmQuery};
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20QueryMsg};

use crate::mock::{create_http_mock, DefaultWasmMockQuerier, HttpWasmMockQuerier};

const TERRA2_PULBLIC_NODE_URL: &str = "https://phoenix-lcd.terra.dev";

#[test]
pub fn simple_query() {
    let contract_pair_astro_usdc =
        String::from("terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6");
    let contract_astro =
        String::from("terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26");

    let deps: OwnedDeps<_, _, HttpWasmMockQuerier<DefaultWasmMockQuerier>> =
        create_http_mock(None, TERRA2_PULBLIC_NODE_URL, None);

    let amount =
        query_balance_cw20(deps.as_ref(), contract_astro, contract_pair_astro_usdc).unwrap();

    println!("{amount}")
}

pub fn query_balance_cw20(
    deps: Deps,
    cw20_address: String,
    account_address: String,
) -> StdResult<Uint128> {
    let balance_response: Cw20BalanceResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: cw20_address,
            msg: to_binary(&Cw20QueryMsg::Balance {
                address: account_address,
            })?,
        }))?;

    Ok(balance_response.balance)
}
