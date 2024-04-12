use std::collections::HashMap;

use anyhow::Ok;
use arbiter_engine::{
    machine::{Processor, State},
    messager::Message,
};
use ethers::utils::parse_ether;
use tracing::debug;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token_data: Vec<TokenData>,
}

impl State for Config {
    type Data = Self;
}

#[derive(Debug, Clone)]
pub struct Processing {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub tokens: HashMap<String, (TokenData, ArbiterToken<ArbiterMiddleware>)>,
}

impl State for Processing {
    type Data = Self;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Response {
    Success,
    Failed,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenAdmin<S: State> {
    pub data: S::Data,
}

impl TokenAdmin<Processing> {
    async fn reply_token_data(&self, token_name: String, to: String) -> Result<()> {
        let token_data = &self.data.tokens.get(&token_name).unwrap().0;
        Ok(self.data.messager.send(To::Agent(to), token_data).await?)
    }
    async fn reply_address_of(&self, token_name: String, to: String) -> Result<()> {
        let token_address = self.data.tokens.get(&token_name).unwrap().1.address();
        Ok(self
            .data
            .messager
            .send(To::Agent(to), token_address)
            .await?)
    }

    async fn reply_get_asset_universe(&self, to: String) -> Result<()> {
        let asset_universe = self
            .data
            .tokens
            .values()
            .map(|(meta, token)| (meta, token.address()))
            .collect::<Vec<(&TokenData, eAddress)>>();

        Ok(self
            .data
            .messager
            .send(To::Agent(to), asset_universe)
            .await?)
    }

    async fn reply_mint_request(&self, mint_request: MintRequest, to: String) -> Result<()> {
        println!("Got to here in mint request");
        let token = &self.data.tokens.get(&mint_request.token).unwrap().1;
        token
            .mint(
                mint_request.mint_to,
                parse_ether(mint_request.mint_amount).unwrap(),
            )
            .send()
            .await?
            .await?;
        println!("Made the mint call to RPC in mint request");
        Ok(self
            .data
            .messager
            .send(To::Agent(to), Response::Success)
            .await?)
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
impl Behavior<Message> for TokenAdmin<Config> {
    type Processor = TokenAdmin<Processing>;
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
        let mut tokens = HashMap::new();
        let mut token_data_hashmap = HashMap::new();
        for token_data in &self.data.token_data.clone() {
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
            tokens.insert(token_data.name.clone(), (token_data.clone(), token));
        }
        debug!("Tokens deployed!");

        let process = Self::Processor {
            data: Processing {
                messager,
                client,
                tokens,
            },
        };
        let stream = process.data.messager.clone().stream()?;
        Ok(Some((process, stream)))
    }
}

// TODO: We could make this a `MessageDecode<T>` stream to make life a little
// easier. Would be nice to add this in arbiter_engine.
#[async_trait::async_trait]
impl Processor<Message> for TokenAdmin<Processing> {
    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let query: TokenAdminQuery =
            serde_json::from_str(&event.data).unwrap_or(TokenAdminQuery::NoOp);
        match query {
            TokenAdminQuery::AddressOf(token_name) => {
                self.reply_address_of(token_name, event.from).await?;
            }
            TokenAdminQuery::MintRequest(mint_request) => {
                self.reply_mint_request(mint_request, event.from).await?;
            }
            TokenAdminQuery::GetAssetUniverse => {
                self.reply_get_asset_universe(event.from).await?;
            }
            TokenAdminQuery::GetTokenData(token_name) => {
                self.reply_token_data(token_name, event.from).await?;
            }
            TokenAdminQuery::NoOp => {
                debug!("NoOp");
            }
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
        pool::constant_sum::{ConstantSumConfig, ConstantSumPool},
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
