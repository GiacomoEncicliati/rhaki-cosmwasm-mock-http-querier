use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::marker::PhantomData;

use base64::{engine::general_purpose, Engine as _};
use cosmwasm_std::{
    from_slice,
    testing::{MockApi, MockQuerier, MockStorage},
    Binary, Coin, ContractResult, CustomQuery, OwnedDeps, Querier, QuerierResult, QueryRequest,
    SystemError, SystemResult, WasmQuery,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct LcdSmartQueryHttpResponse {
    data: Value,
}

pub struct HttpWasmMockQuerier<C: CustomQuery + DeserializeOwned> {
    base: MockQuerier<C>,
    url_lcd: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DefaultWasmMockQuerier {}

impl CustomQuery for DefaultWasmMockQuerier {}

impl<C: CustomQuery + DeserializeOwned> HttpWasmMockQuerier<C> {
    pub fn new(
        base: Option<MockQuerier<C>>,
        url_lcd: String,
        balances: Option<&[(&str, &[Coin])]>,
    ) -> Self {
        HttpWasmMockQuerier {
            base: base.unwrap_or_else(|| MockQuerier::new(balances.unwrap_or(&[]))),
            url_lcd,
        }
    }

    pub fn handle_query(&self, request: &QueryRequest<C>) -> QuerierResult {
        match request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                
                let mut url = self.url_lcd.clone();
                url.push_str("/cosmwasm/wasm/v1/contract/");
                url.push_str(contract_addr);
                url.push_str("/smart/");
                url.push_str(msg.to_string().as_str());

                let res: LcdSmartQueryHttpResponse =
                    reqwest::blocking::get(url).unwrap().json().unwrap();
                let encoded_resp: String =
                    general_purpose::STANDARD_NO_PAD.encode(res.data.to_string());
                let binary_resp = Binary::from_base64(encoded_resp.as_str()).unwrap();

                QuerierResult::Ok(ContractResult::Ok(binary_resp))
            }
            _ => self.base.handle_query(request),
        }
    }
}

impl<C: CustomQuery + DeserializeOwned> Querier for HttpWasmMockQuerier<C> {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<C> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

pub fn create_http_mock<C: CustomQuery + DeserializeOwned>(
    custom_mock: Option<MockQuerier<C>>,
    url_lcd: &str,
    balances: Option<&[(&str, &[Coin])]>,
) -> OwnedDeps<MockStorage, MockApi, HttpWasmMockQuerier<C>> {
    let querier: HttpWasmMockQuerier<C> =
        HttpWasmMockQuerier::new(custom_mock, String::from(url_lcd), balances);

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier,
        custom_query_type: PhantomData::default(),
    }
}
