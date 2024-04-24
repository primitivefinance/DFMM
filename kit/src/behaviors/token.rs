use std::collections::HashMap;
use super::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenAdmin<S: State> {
    pub data: S::Data,
}

#[derive(Debug, Clone, Serialize, Deserialize, State)]
pub struct Config {
    pub token_data: Vec<TokenData>,
}

#[derive(Debug, Clone, State)]
pub struct Processing {
    pub messager: Messager,
    pub client: Arc<ArbiterMiddleware>,
    pub tokens: HashMap<String, (TokenData, ArbiterToken<ArbiterMiddleware>)>,
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin<Config> {
    type Processor = TokenAdmin<Processing>;
    async fn startup(
        mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Self::Processor> {
        let mut tokens = HashMap::new();
        for token_data in self.data.token_data.drain(..) {
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
            tokens.insert(token_data.name.clone(), (token_data, token));
        }

        debug!("Tokens deployed {:#?}", tokens);

        Ok(Self::Processor {
            data: Processing {
                messager,
                client,
                tokens,
            },
        })
    }
}

// TODO: We could make this a `MessageDecode<T>` stream to make life a little
// easier. Would be nice to add this in arbiter_engine.
#[async_trait::async_trait]
impl Processor<Message> for TokenAdmin<Processing> {
    async fn get_stream(&mut self) -> Result<Option<EventStream<Message>>> {
        Ok(Some(self.data.messager.stream()?))
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        match serde_json::from_str(&event.data) {
            Ok(TokenAdminQuery::AddressOf(token_name)) => {
                self.reply_address_of(token_name, event.from).await?;
            }
            Ok(TokenAdminQuery::MintRequest(mint_request)) => {
                self.reply_mint_request(mint_request, event.from).await?;
            }
            Ok(TokenAdminQuery::GetAssetUniverse) => {
                self.reply_get_asset_universe(event.from).await?;
            }
            Ok(TokenAdminQuery::GetTokenData(token_name)) => {
                self.reply_token_data(token_name, event.from).await?;
            }
            _ => {
                debug!("TokenAdmin got some other message variant it could ignore.");
            }
        }
        Ok(ControlFlow::Continue)
    }
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
            .map(|(meta, token)| (meta.clone(), token.address()))
            .collect::<Vec<(TokenData, eAddress)>>();

        Ok(self
            .data
            .messager
            .send(To::Agent(to), asset_universe)
            .await?)
    }

    async fn reply_mint_request(&self, mint_request: MintRequest, to: String) -> Result<()> {
        let token = &self.data.tokens.get(&mint_request.token).unwrap().1;
        token
            .mint(mint_request.mint_to, mint_request.mint_amount)
            .send()
            .await?
            .await?;
        debug!("Made the mint call to RPC in mint request");
        Ok(self
            .data
            .messager
            .send(To::Agent(to), Response::Success)
            .await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<eAddress>,
}

/// Used as an action to ask what tokens are available.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TokenAdminQuery {
    AddressOf(String),
    MintRequest(MintRequest),
    GetAssetUniverse,
    GetTokenData(String),
}

/// Used as an action to mint tokens.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintRequest {
    /// The token to mint.
    pub token: String,

    /// The address to mint to.
    pub mint_to: eAddress,

    /// The amount to mint.
    pub mint_amount: eU256,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Response {
    Success,
    Failed,
}
