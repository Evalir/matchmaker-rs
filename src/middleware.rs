use ethers::{providers::Middleware, signers::Signer, types::Bytes};
use ethers_flashbots::Relay;
use url::Url;

use crate::share::ShareTransactionOptions;

#[derive(Debug)]
pub struct MevShareMiddleware<M, S> {
    #[allow(dead_code)] // For now
    inner: M,
    relay: Relay<S>,
    simulation_relay: Option<Relay<S>>,
}

impl<M: Middleware, S: Signer> MevShareMiddleware<M, S> {
    pub fn new(inner: M, relay_url: impl Into<Url>, relay_signer: S) -> Self {
        Self {
            inner,
            relay: Relay::new(relay_url, Some(relay_signer)),
            simulation_relay: None,
        }
    }

    /// Get the relay client used by the middleware.
    pub fn relay(&self) -> &Relay<S> {
        &self.relay
    }

    /// Get the relay client used by the middleware to simulate
    /// bundles if set.
    pub fn simulation_relay(&self) -> Option<&Relay<S>> {
        self.simulation_relay.as_ref()
    }

    /// Set a separate relay to use for simulating bundles.
    ///
    /// This can either be a full Flashbots relay or a node that implements
    /// the `eth_callBundle` remote procedure call.
    pub fn set_simulation_relay(&mut self, relay_url: impl Into<Url>) {
        self.simulation_relay = Some(Relay::new(relay_url, None));
    }

    pub fn send_share_tx(&self, signed_tx: Bytes, opts: ShareTransactionOptions) {
        todo!()
    }
}
