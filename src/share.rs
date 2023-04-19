use ethers::types::{Bytes, U256};
use serde::Serialize;

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize)]
pub struct ShareTransactionOptions {
    pub max_block_number: Option<U256>,
    #[serde(flatten)]
    pub preferences: FlashbotsHints,
}

impl ShareTransactionOptions {
    pub fn new(max_block_number: Option<U256>, preferences: FlashbotsHints) -> Self {
        Self {
            max_block_number,
            preferences,
        }
    }

    pub fn with_max_block_number(&self, max_block_number: U256) -> Self {
        Self {
            max_block_number: Some(max_block_number),
            preferences: self.preferences.clone(),
        }
    }

    pub fn with_logs(&self) -> Self {
        Self {
            max_block_number: self.max_block_number,
            preferences: FlashbotsHints {
                logs: true,
                calldata: self.preferences.calldata,
                function_selector: self.preferences.function_selector,
                contract_address: self.preferences.contract_address,
            },
        }
    }

    pub fn with_calldata(&self) -> Self {
        Self {
            max_block_number: self.max_block_number,
            preferences: FlashbotsHints {
                logs: self.preferences.logs,
                calldata: true,
                function_selector: self.preferences.function_selector,
                contract_address: self.preferences.contract_address,
            },
        }
    }

    pub fn with_function_selector(&self) -> Self {
        Self {
            max_block_number: self.max_block_number,
            preferences: FlashbotsHints {
                logs: self.preferences.logs,
                calldata: self.preferences.calldata,
                function_selector: true,
                contract_address: self.preferences.contract_address,
            },
        }
    }

    pub fn with_contract_address(&self) -> Self {
        Self {
            max_block_number: self.max_block_number,
            preferences: FlashbotsHints {
                logs: self.preferences.logs,
                calldata: self.preferences.calldata,
                function_selector: self.preferences.function_selector,
                contract_address: true,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InternalFlashbotsPayload {
    pub signed_tx: Bytes,
    pub max_block_number: Option<U256>,
    pub preferences: InternalFlashbotsPayloadPreferences,
}

#[derive(Debug, Serialize)]
pub struct InternalFlashbotsPayloadPreferences {
    fast: bool,
    auction: Option<FlashbotsAuctionPreferences>,
}

impl From<ShareTransactionOptions> for InternalFlashbotsPayloadPreferences {
    fn from(opts: ShareTransactionOptions) -> Self {
        let enable =
            opts.max_block_number.is_some() || opts.preferences != FlashbotsHints::default();

        Self {
            fast: true,
            auction: match enable {
                true => Some(FlashbotsAuctionPreferences {
                    enable,
                    hint: Some(opts.preferences),
                }),
                false => None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FlashbotsAuctionPreferences {
    enable: bool,
    hint: Option<FlashbotsHints>,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize)]
pub struct FlashbotsHints {
    pub logs: bool,
    pub calldata: bool,
    pub function_selector: bool,
    pub contract_address: bool,
}
