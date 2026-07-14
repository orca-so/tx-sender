use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::hash::Hash;
use std::time::Duration;

const MAINNET_HASH: &str = "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d";
const DEVNET_HASH: &str = "EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG";
const ECLIPSE_HASH: &str = "EAQLJCV2mh23BsK2P9oYpV5CHVLDNHTxYss3URrNmg3s";
const ECLIPSE_TESTNET_HASH: &str = "CX4huckiV9QNAkKNVKi5Tj8nxzBive5kQimd94viMKsU";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainId {
    Mainnet,
    Devnet,
    Eclipse,
    EclipseTestnet,
    Unknown(Hash),
}

impl ChainId {
    pub fn is_mainnet(&self) -> bool {
        matches!(self, Self::Mainnet)
    }
}

impl From<Hash> for ChainId {
    fn from(hash: Hash) -> Self {
        // Convert hash to string once for comparison
        let hash_str = hash.to_string();
        // Compare with string constants directly
        if hash_str == MAINNET_HASH {
            return Self::Mainnet;
        } else if hash_str == DEVNET_HASH {
            return Self::Devnet;
        } else if hash_str == ECLIPSE_HASH {
            return Self::Eclipse;
        } else if hash_str == ECLIPSE_TESTNET_HASH {
            return Self::EclipseTestnet;
        }
        Self::Unknown(hash)
    }
}

/// RPC configuration for connecting to Solana nodes.
///
/// Start with [`RpcConfig::default`] and use the `with_*` methods to override
/// individual settings, or use [`RpcConfig::new`] to detect the chain from an
/// RPC endpoint.
#[non_exhaustive]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RpcConfig {
    pub url: String,
    pub supports_priority_fee_percentile: bool,
    pub chain_id: Option<ChainId>,
}

impl RpcConfig {
    /// Set the RPC URL without contacting the endpoint or detecting its chain.
    /// Prefer [`RpcConfig::new`] when the chain id is not already known.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn with_priority_fee_percentile_support(
        mut self,
        supports_priority_fee_percentile: bool,
    ) -> Self {
        self.supports_priority_fee_percentile = supports_priority_fee_percentile;
        self
    }

    pub fn with_chain_id(mut self, chain_id: ChainId) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    pub async fn new(url: impl Into<String>) -> Result<Self, String> {
        let url = url.into();
        let client = RpcClient::new(url.clone());
        let genesis_hash = client
            .get_genesis_hash()
            .await
            .map_err(|e| format!("Chain Detection Error: Failed to get genesis hash: {e}"))?;

        Ok(Self {
            url,
            supports_priority_fee_percentile: false,
            chain_id: Some(ChainId::from(genesis_hash)),
        })
    }

    pub fn client(&self) -> RpcClient {
        RpcClient::new_with_timeout(self.url.clone(), Duration::from_millis(90_000))
    }

    /// Check if the RPC is connected to Solana mainnet
    pub fn is_mainnet(&self) -> bool {
        self.chain_id
            .as_ref()
            .is_some_and(|chain_id| chain_id.is_mainnet())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_rpc_configuration() {
        let config = RpcConfig::default()
            .with_url("https://example.com")
            .with_priority_fee_percentile_support(true)
            .with_chain_id(ChainId::Mainnet);

        assert_eq!(config.url, "https://example.com");
        assert!(config.supports_priority_fee_percentile);
        assert_eq!(config.chain_id, Some(ChainId::Mainnet));
    }
}
