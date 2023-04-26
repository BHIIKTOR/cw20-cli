use cosmwasm_std::Uint128;
use cw_orc::{networks::ChainInfo, queriers::Node, *};
use tokio::runtime::Runtime;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use std::{env, sync::Arc};

use uid::Id as IdT;

#[derive(Copy, Clone, Eq, PartialEq)]
struct DeployId(());

type Id = IdT<DeployId>;

const CW20_CONTRACT_WASM: &str = "artifacts/cw20_base.wasm";

struct TxWrapper(TxResponse<Daemon>);

impl Serialize for TxWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut state = serializer.serialize_struct("CosmTxResponse", 12)?;
        state.serialize_field("height", &self.0.height)?;
        state.serialize_field("txhash", &self.0.txhash)?;
        state.serialize_field("codespace", &self.0.codespace)?;
        state.serialize_field("code", &self.0.code)?;
        state.serialize_field("data", &self.0.data)?;
        state.serialize_field("raw_log", &self.0.raw_log)?;
        state.serialize_field("logs", &self.0.logs)?;
        state.serialize_field("info", &self.0.info)?;
        state.serialize_field("gas_wanted", &self.0.gas_wanted)?;
        state.serialize_field("gas_used", &self.0.gas_used)?;
        // state.serialize_field("timestamp", &self.0.timestamp)?;
        // state.serialize_field("events", &self.0.events)?;

        state.end()
    }
}

#[contract(
    cw20_base::msg::InstantiateMsg,
    cw20_base::msg::ExecuteMsg,
    cw20_base::msg::QueryMsg,
    cw20_base::msg::MigrateMsg
)]
pub struct Cw20;

#[derive(Clone)]
pub struct MyContractoor {
    pub inner: Cw20<Daemon>,
    sender: cosmwasm_std::Addr,
    daemon: Daemon,
}

impl Uploadable<Mock> for Cw20<Mock> {
    fn source(&self) -> <Mock as cw_orc::TxHandler>::ContractSource {
        Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate(cw20_base::contract::migrate),
        )
    }
}

impl Uploadable<Daemon> for Cw20<Daemon> {
    fn source(&self) -> <Daemon as cw_orc::TxHandler>::ContractSource {
        // create contract base configuration
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/{}", crate_path, CW20_CONTRACT_WASM);
        WasmPath::new(wasm_path).unwrap()
    }
}

impl MyContractoor {
    pub fn new(
        runtime: &Runtime,
        contract_id: &str,
        chain: ChainInfo,
    ) -> Self {
        let id = Id::new();

        let res = Daemon::builder()
            .chain(chain)
            .handle(runtime.handle())
            .mnemonic(env::var("LOCAL_MNEMONIC").unwrap())
            .build();

        let Some(daemon) = res.as_ref().ok() else {
            panic!("Error: {}", res.err().unwrap().to_string());
        };

        let sender = daemon.sender.address().unwrap();

        let contract = Cw20(Contract::new(
            format!("{}:{}", contract_id, id),
            daemon.clone(),
        ));

        Self {
            sender,
            inner: contract,
            daemon: daemon.clone(),
        }
    }

    pub fn init(
        &self,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<cw_orc::TxResponse<Daemon>, CwOrcError> {
        self.inner.instantiate(
            &cw20_base::msg::InstantiateMsg {
                name,
                symbol,
                decimals,
                initial_balances: vec![],
                mint: None,
                marketing: None,
            },
            Some(&self.sender),
            None,
        )
    }

    pub fn transfer(
        &self,
        recipient: impl Into<String>,
        amount: impl Into<Uint128>,
    ) -> Result<cw_orc::TxResponse<Daemon>, CwOrcError> {
        self.inner.execute(
            &cw20_base::msg::ExecuteMsg::Transfer {
                recipient: recipient.into(),
                amount: amount.into(),
            },
            None,
        )
    }

    pub fn find_tx(&self, hash: String) -> Result<cw_orc::TxResponse<Daemon>, cw_orc::DaemonError> {
        let rt = Arc::new(Runtime::new().unwrap());
        let querier: Node = self.daemon.query();
        rt.block_on(querier.find_tx_by_hash(hash))
    }
}

#[cfg(test)]
mod my_contract_testoor {
    // use speculoos::prelude::*;
    use super::MyContractoor;
    use cw_orc::networks;
    use dotenvy::dotenv;
    use std::{env, sync::Arc};
    use tokio::runtime::Runtime;

    #[test]
    fn general() {
        pretty_env_logger::init();
        dotenv().ok();

        log::info!(
            "Using LOCAL_MNEMONIC: {}",
            env::var("LOCAL_MNEMONIC").unwrap()
        );

        let runtime = Arc::new(Runtime::new().unwrap());

        let _ = MyContractoor::new(
            &runtime,
            "test-name",
            networks::LOCAL_JUNO,
        );
    }
}
