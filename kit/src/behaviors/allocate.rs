use std::marker::PhantomData;
use arbiter_core::events::stream_event;
use arbiter_engine::machine::ControlFlow;

use self::{pool::PoolType, token_admin::{MintRequest, TokenAdminQuery}};

use super::*;

trait AllocationType<E> {
    fn change_allocation_amount(&self, event: E) -> Option<eU256>;
}

// not sure what to specify for E right now but this is the general idea so far
struct ChangeAllocation<P: PoolType, A: AllocationType<E>> {
    pool: P,
    client: Arc<ArbiterMiddleware>,
    allocation_data: P::AllocationData,
    allocation_type: A,
    phantom: PhantomData<E>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitialAllocation<P: PoolType> {
    /// The initial amount of token X.
    pub initial_x: eU256,
    /// The initial Price
    pub initial_price: eU256,
    /// Initial Parameters
    pub initial_parameters: P::AllocationData,
    /// The tokens to allocate
    pub tokens: (ArbiterToken<ArbiterMiddleware>, ArbiterToken<ArbiterMiddleware>),
    /// The tokens to request.
    pub token_data: TokenData,
    /// The agent ID to request tokens to.
    pub request_to: String,
    /// Client to have an address to receive token mint to and check balance
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
    /// The messaging layer for the token requester.
    #[serde(skip)]
    pub messager: Option<Messager>,
}


#[allow(unused_variables)]
#[async_trait::async_trait]
impl<P: PoolType> Behavior<()> for InitialAllocation<P> {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        messager
            .send(
                To::Agent(self.request_to.clone()),
                &TokenAdminQuery::AddressOf(self.token_data.name.clone()),
            )
            .await?;
        let message = messager.get_next().await.unwrap();
        let token_address = serde_json::from_str::<eAddress>(&message.data).unwrap();
        let token = ArbiterToken::new(token_address, client.clone());
        self.token_data.address = Some(token_address);

        let mint_data = TokenAdminQuery::MintRequest(MintRequest {
            token: self.token_data.name.clone(),
            mint_to: client.address(),
            mint_amount: 1,
        });
        messager
            .send(To::Agent(self.request_to.clone()), mint_data)
            .await?;

        self.messager = Some(messager.clone());
        self.client = Some(client.clone());
        let transfer_stream = stream_event(token.transfer_filter());
        Ok(Some(transfer_stream))
    }
}
