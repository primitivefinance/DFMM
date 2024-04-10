// use std::collections::HashMap;
// use arbiter_engine::machine::Processor;

// use super::*;

// #[derive(Deserialize, Clone, Debug)]
// pub struct TokenAdmin {
//     /// The identifier of the token admin.
//     pub token_data: HashMap<String, TokenData>,
//     pub tokens: Option<HashMap<String, ArbiterToken<ArbiterMiddleware>>>,
//     pub client: Option<Arc<ArbiterMiddleware>>,
//     pub messager: Option<Messager>,
//     pub count: u64,
//     pub max_count: Option<u64>,
// }

// /// Used as an action to ask what tokens are available.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum TokenAdminQuery {
//     /// Get the address of the token.
//     AddressOf(String),

//     /// Mint tokens.
//     MintRequest(MintRequest),
// }

// /// Used as an action to mint tokens.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct MintRequest {
//     /// The token to mint.
//     pub token: String,

//     /// The address to mint to.
//     pub mint_to: eAddress,

//     /// The amount to mint.
//     pub mint_amount: u64,
// }

// #[async_trait::async_trait]
// impl Behavior<Message> for TokenAdmin {
//     type Processor = TokenAdmin;

//     #[tracing::instrument(skip(self), fields(id = messager.id.as_deref()))]
//     async fn startup(
//         &mut self,
//         client: Arc<ArbiterMiddleware>,
//         messager: Messager,
//     ) -> Result<Option<(Self::Processor, EventStream<Message>)>> {
//         self.messager = Some(messager.clone());
//         self.client = Some(client.clone());
//         for token_data in self.token_data.values_mut() {
//             let token = ArbiterToken::deploy(
//                 client.clone(),
//                 (
//                     token_data.name.clone(),
//                     token_data.symbol.clone(),
//                     token_data.decimals,
//                 ),
//             )
//             .unwrap()
//             .send()
//             .await
//             .unwrap();

//             token_data.address = Some(token.address());
//             self.tokens
//                 .get_or_insert_with(HashMap::new)
//                 .insert(token_data.name.clone(), token.clone());
//         }
//         let thing = self.messager.unwrap().stream()?;
//         Ok(Some((self.clone(), thing)))
//     }
// }

// #[async_trait::async_trait]
// impl Processor<Message> for TokenAdmin {
//     async fn process(&mut self, event: Message) -> Result<ControlFlow> {
//         if self.tokens.is_none() {
//             error!(
//                 "There were no tokens to deploy! You must add tokens to the token admin before running the simulation."
//             );
//         }
//         let query: TokenAdminQuery = serde_json::from_str(&event.data).unwrap();
//         trace!("Got query: {:?}", query);
//         let messager = self.messager.as_ref().unwrap();
//         match query {
//             TokenAdminQuery::AddressOf(token_name) => {
//                 trace!(
//                     "Getting address of token with name: {:?}",
//                     token_name.clone()
//                 );
//                 let token_data = self.token_data.get(&token_name).unwrap();
//                 messager
//                     .send(To::Agent(event.from.clone()), token_data.address)
//                     .await?;
//             }
//             TokenAdminQuery::MintRequest(mint_request) => {
//                 trace!("Minting tokens: {:?}", mint_request);
//                 let token = self
//                     .tokens
//                     .as_ref()
//                     .unwrap()
//                     .get(&mint_request.token)
//                     .unwrap();
//                 token
//                     .mint(mint_request.mint_to, eU256::from(mint_request.mint_amount))
//                     .send()
//                     .await
//                     .unwrap()
//                     .await
//                     .unwrap();
//                 self.count += 1;
//                 if self.count == self.max_count.unwrap_or(u64::MAX) {
//                     warn!("Reached max count. Halting behavior.");
//                     return Ok(ControlFlow::Halt);
//                 }
//             }
//         }
//         Ok(ControlFlow::Continue)
//     }
// }
