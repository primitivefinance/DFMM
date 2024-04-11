use std::collections::HashMap;

use anyhow::Ok;
use arbiter_engine::{
    machine::{Processing, Processor, State},
    messager::Message,
};
use ethers::utils::parse_ether;
use tracing::debug;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAdminConfig {
    pub token_data: Vec<TokenData>,
}

#[derive(Debug, Clone)]
pub struct TokenAdminProcessing {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub tokens: HashMap<String, ArbiterToken<ArbiterMiddleware>>,
    pub token_data: HashMap<String, (TokenData, eAddress)>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct TokenAdmin<S: State> {
    pub data: S::Data,
}

impl TokenAdmin<Processing<TokenAdminProcessing>> {
    async fn reply_token_data(&self, token_name: String, to: String) {
        let token_data = self.data.token_data.get(&token_name).unwrap();
        let _ = self.data.messager.send(To::Agent(to), token_data).await;
    }
    async fn reply_address_of(&self, token_name: String, to: String) {
        let (_, token_address) = self.data.token_data.get(&token_name).unwrap();
        let _ = self.data.messager.send(To::Agent(to), token_address).await;
    }

    async fn reply_get_asset_universe(&self, to: String) {
        let asset_universe = self
            .data
            .token_data
            .values()
            .cloned()
            .collect::<Vec<(TokenData, eAddress)>>();

        let _ = self.data.messager.send(To::Agent(to), asset_universe).await;
    }

    async fn reply_mint_request(&self, mint_request: MintRequest, _to: String) {
        let token = self.data.tokens.get(&mint_request.token).unwrap();
        token
            .mint(
                mint_request.mint_to,
                parse_ether(mint_request.mint_amount).unwrap(),
            )
            .send()
            .await
            .unwrap();
    }
}

/// Used as an action to ask what tokens are available.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TokenAdminQuery {
    AddressOf(String),
    MintRequest(MintRequest),
    GetAssetUniverse,
    GetTokenData(String),
    NoOp,
}
// Result<Option<(Self::Processor, EventStream<E>)
#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin<Configuration<TokenAdminConfig>> {
    type Processor = TokenAdmin<Processing<TokenAdminProcessing>>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
        let mut tokens = HashMap::new();
        let mut token_data_hashmap = HashMap::new();
        for token_data in &self.data.token_data.clone() {
            // let mut token_data = token_data.clone();
            let token = ArbiterToken::deploy(
                client.clone(),
                (
                    token_data.name.clone(),
                    token_data.symbol.clone(),
                    token_data.decimals,
                ),
            )
            .unwrap()
            .send()
            .await
            .unwrap();
            token_data_hashmap.insert(
                token_data.name.clone(),
                (token_data.clone(), token.address()),
            );
            tokens.insert(token_data.name.clone(), token.clone());
        }
        debug!("Tokens deployed!");

        let process = Self::Processor {
            data: TokenAdminProcessing {
                messager,
                client,
                tokens,
                token_data: token_data_hashmap,
            },
        };
        let stream = process.data.messager.clone().stream()?;
        Ok(Some((process, stream)))
    }
}

#[async_trait::async_trait]
impl Processor<Message> for TokenAdmin<Processing<TokenAdminProcessing>> {
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let query: TokenAdminQuery =
            serde_json::from_str(&event.data).unwrap_or(TokenAdminQuery::NoOp);
        match query {
            TokenAdminQuery::AddressOf(token_name) => {
                self.reply_address_of(token_name, event.from).await;
            }
            TokenAdminQuery::MintRequest(mint_request) => {
                self.reply_mint_request(mint_request, event.from).await;
            }
            TokenAdminQuery::GetAssetUniverse => {
                self.reply_get_asset_universe(event.from).await;
            }
            TokenAdminQuery::GetTokenData(token_name) => {
                self.reply_token_data(token_name, event.from).await;
            }
            TokenAdminQuery::NoOp => {}
        }
        Ok(ControlFlow::Continue)
    }
}

mod test {
    use std::{str::FromStr, sync::WaitTimeoutResult};

    use arbiter_engine::{agent::Agent, world::World};
    use ethers::types::Address;
    use futures_util::StreamExt;
    use tracing::{level_filters::LevelFilter, Level};
    use tracing_subscriber::FmtSubscriber;

    use self::{
        bindings::{constant_sum_solver::ConstantSumParams, usdc::USDC},
        pool::constant_sum::{ConstantSumInitData, ConstantSumPool},
    };
    use super::*;
    use crate::behaviors::behaviors::TokenAdmin;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn token_admin_behavior_test() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .pretty()
            .finish();
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let mut world = World::new("test");
        let agent = Agent::builder("token_admin_agent");
        let token_admin_behavior = default_admin_config();
        world.add_agent(agent.with_behavior(token_admin_behavior));

        world.run().await.unwrap();
    }
}
