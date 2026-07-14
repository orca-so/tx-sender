#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Percentile {
    P25,
    P50,
    P75,
    P95,
    P99,
}

impl Percentile {
    pub fn as_value(&self) -> u8 {
        match self {
            Self::P25 => 25,
            Self::P50 => 50,
            Self::P75 => 75,
            Self::P95 => 95,
            Self::P99 => 99,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitoPercentile {
    P25,
    P50,
    P50Ema, // 50th percentile exponential moving average
    P75,
    P95,
    P99,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PriorityFeeStrategy {
    Dynamic {
        percentile: Percentile,
        max_lamports: u64,
    },
    Exact(u64),
    Disabled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JitoFeeStrategy {
    Dynamic {
        percentile: JitoPercentile,
        max_lamports: u64,
    },
    Exact(u64),
    Disabled,
}

/// Priority fee, compute-unit margin, and Jito tip settings.
///
/// Start with [`FeeConfig::default`] and use the `with_*` methods to override
/// individual settings.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct FeeConfig {
    pub priority_fee: PriorityFeeStrategy,
    pub jito: JitoFeeStrategy,
    pub compute_unit_margin_multiplier: f64,
    pub jito_block_engine_url: String,
}

impl FeeConfig {
    pub fn with_priority_fee(mut self, priority_fee: PriorityFeeStrategy) -> Self {
        self.priority_fee = priority_fee;
        self
    }

    pub fn with_jito(mut self, jito: JitoFeeStrategy) -> Self {
        self.jito = jito;
        self
    }

    pub fn with_compute_unit_margin_multiplier(
        mut self,
        compute_unit_margin_multiplier: f64,
    ) -> Self {
        self.compute_unit_margin_multiplier = compute_unit_margin_multiplier;
        self
    }

    pub fn with_jito_block_engine_url(mut self, jito_block_engine_url: impl Into<String>) -> Self {
        self.jito_block_engine_url = jito_block_engine_url.into();
        self
    }
}

impl Default for FeeConfig {
    fn default() -> Self {
        Self {
            priority_fee: PriorityFeeStrategy::Disabled,
            jito: JitoFeeStrategy::Disabled,
            compute_unit_margin_multiplier: 1.1,
            jito_block_engine_url: "https://bundles.jito.wtf".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_fee_configuration() {
        let config = FeeConfig::default()
            .with_priority_fee(PriorityFeeStrategy::Exact(10))
            .with_jito(JitoFeeStrategy::Exact(20))
            .with_compute_unit_margin_multiplier(1.25)
            .with_jito_block_engine_url("https://example.com");

        assert_eq!(config.priority_fee, PriorityFeeStrategy::Exact(10));
        assert_eq!(config.jito, JitoFeeStrategy::Exact(20));
        assert_eq!(config.compute_unit_margin_multiplier, 1.25);
        assert_eq!(config.jito_block_engine_url, "https://example.com");
    }
}
