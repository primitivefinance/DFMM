use std::collections::HashMap;

use super::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenAdmin {
    /// The identifier of the token admin.
    pub token_data: HashMap<String, TokenData>,
    #[serde(skip)]
    pub tokens: Option<HashMap<String, ArbiterToken<ArbiterMiddleware>>>,
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
    #[serde(default)]
    pub count: u64,
    #[serde(default = "default_max_count")]
    pub max_count: Option<u64>,
}

pub fn default_max_count() -> Option<u64> {
    Some(3)
}

/// Used as an action to ask what tokens are available.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TokenAdminQuery {
    /// Get the address of the token.
    AddressOf(String),

    /// Mint tokens.
    MintRequest(MintRequest),
}

/// Used as an action to mint tokens.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintRequest {
    /// The token to mint.
    pub token: String,

    /// The address to mint to.
    pub mint_to: eAddress,

    /// The amount to mint.
    pub mint_amount: u64,
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin {
    #[tracing::instrument(skip(self), fields(id = messager.id.as_deref()))]
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        self.messager = Some(messager.clone());
        self.client = Some(client.clone());
        for token_data in self.token_data.values_mut() {
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

            token_data.address = Some(token.address());
            self.tokens
                .get_or_insert_with(HashMap::new)
                .insert(token_data.name.clone(), token.clone());
        }
        Ok(None)
    }
}
