use crate::fee_config::{FeeConfig, Percentile, PriorityFeeStrategy};
use crate::rpc_config::RpcConfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSimulateTransactionConfig;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_instruction::Instruction;
use solana_message::AddressLookupTableAccount;
use solana_message::{v0::Message, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_rpc_client_api::response::RpcPrioritizationFee;
use solana_transaction::versioned::VersionedTransaction;

/// Compute unit limit strategy to apply when building a transaction.
/// - Dynamic: Estimate compute units by simulating the transaction.
///            If the simulation fails, the transaction will not build.
/// - Exact: Directly use the provided compute unit limit up to 1,400,000
#[derive(Debug, Default)]
pub enum ComputeUnitLimitStrategy {
    #[default]
    Dynamic,
    Exact(u32),
}

pub const MAX_COMPUTE_UNIT_LIMIT: u32 = 1_400_000;

pub(crate) fn apply_compute_unit_margin(compute_units: u32, multiplier: f64) -> u32 {
    ((compute_units as f64 * multiplier) as u32).min(MAX_COMPUTE_UNIT_LIMIT)
}

pub(crate) fn validate_compute_unit_limit_strategy(
    strategy: &ComputeUnitLimitStrategy,
) -> Result<(), String> {
    if let ComputeUnitLimitStrategy::Exact(units) = strategy {
        if !(1..=MAX_COMPUTE_UNIT_LIMIT).contains(units) {
            return Err(format!(
                "Exact compute unit limit must be between 1 and 1,400,000; received {units}"
            ));
        }
    }
    Ok(())
}

/// Compute-unit limit settings used while building a transaction.
///
/// Start with [`ComputeConfig::default`] and use [`ComputeConfig::with_unit_limit`]
/// to override the default dynamic strategy.
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct ComputeConfig {
    pub unit_limit: ComputeUnitLimitStrategy,
}

impl ComputeConfig {
    pub fn with_unit_limit(mut self, unit_limit: ComputeUnitLimitStrategy) -> Self {
        self.unit_limit = unit_limit;
        self
    }
}

pub(crate) fn format_simulation_error(
    err: impl std::fmt::Display,
    logs: Option<Vec<String>>,
) -> String {
    let mut message = format!("Transaction simulation failed: {}", err);

    if let Some(logs) = logs {
        if !logs.is_empty() {
            message.push_str("\nSimulation logs:\n");
            message.push_str(&logs.join("\n"));
        }
    }

    message
}

/// Estimate compute units by simulating a transaction
pub async fn estimate_compute_units(
    rpc_client: &RpcClient,
    instructions: &[Instruction],
    payer: &Pubkey,
    alts: Option<Vec<AddressLookupTableAccount>>,
) -> Result<u32, String> {
    estimate_compute_units_at_commitment(rpc_client, instructions, payer, alts, None, None).await
}

fn simulation_config(
    commitment: Option<solana_commitment_config::CommitmentConfig>,
    min_context_slot: Option<u64>,
) -> RpcSimulateTransactionConfig {
    RpcSimulateTransactionConfig {
        sig_verify: false,
        replace_recent_blockhash: true,
        commitment,
        min_context_slot,
        ..Default::default()
    }
}

pub(crate) async fn estimate_compute_units_at_commitment(
    rpc_client: &RpcClient,
    instructions: &[Instruction],
    payer: &Pubkey,
    alts: Option<Vec<AddressLookupTableAccount>>,
    commitment: Option<solana_commitment_config::CommitmentConfig>,
    min_context_slot: Option<u64>,
) -> Result<u32, String> {
    let alt_accounts = alts.unwrap_or_default();
    let blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(|e| format!("Failed to get recent blockhash: {}", e))?;

    // Add max compute unit limit instruction so that the simulation does not fail
    let mut simulation_instructions =
        vec![ComputeBudgetInstruction::set_compute_unit_limit(1_400_000)];
    simulation_instructions.extend_from_slice(instructions);

    let message = Message::try_compile(payer, &simulation_instructions, &alt_accounts, blockhash)
        .map_err(|e| format!("Failed to compile message: {}", e))?;

    let transaction = VersionedTransaction {
        signatures: vec![
            solana_signature::Signature::default();
            message.header.num_required_signatures.into()
        ],
        message: VersionedMessage::V0(message),
    };

    let result = rpc_client
        .simulate_transaction_with_config(
            &transaction,
            simulation_config(commitment, min_context_slot),
        )
        .await;

    match result {
        Ok(simulation_result) => {
            if let Some(err) = simulation_result.value.err {
                return Err(format_simulation_error(err, simulation_result.value.logs));
            }
            match simulation_result.value.units_consumed {
                Some(units) => Ok(units as u32),
                None => Err("Transaction simulation didn't return consumed units".to_string()),
            }
        }
        Err(e) => Err(format!("Transaction simulation failed: {}", e)),
    }
}

pub(crate) async fn build_compute_budget_instructions(
    rpc_client: &RpcClient,
    instructions: &[Instruction],
    payer: &Pubkey,
    address_lookup_tables: Option<Vec<AddressLookupTableAccount>>,
    compute_config: &ComputeConfig,
    fee_config: &FeeConfig,
    rpc_config: &RpcConfig,
    min_context_slot: Option<u64>,
) -> Result<Vec<Instruction>, String> {
    let writable_accounts = get_writable_accounts(instructions);

    let compute_units = match &compute_config.unit_limit {
        ComputeUnitLimitStrategy::Dynamic => {
            let estimated_compute_units = estimate_compute_units_at_commitment(
                rpc_client,
                instructions,
                payer,
                address_lookup_tables,
                Some(rpc_client.commitment()),
                min_context_slot,
            )
            .await?;

            apply_compute_unit_margin(
                estimated_compute_units,
                fee_config.compute_unit_margin_multiplier,
            )
        }
        ComputeUnitLimitStrategy::Exact(units) => {
            validate_compute_unit_limit_strategy(&compute_config.unit_limit)?;
            *units
        }
    };

    get_compute_budget_instruction(
        rpc_client,
        compute_units,
        payer,
        rpc_config,
        fee_config,
        &writable_accounts,
    )
    .await
}

/// Calculate and return compute budget instructions for a transaction
pub async fn get_compute_budget_instruction(
    client: &RpcClient,
    compute_units: u32,
    _payer: &Pubkey,
    rpc_config: &RpcConfig,
    fee_config: &FeeConfig,
    writable_accounts: &[Pubkey],
) -> Result<Vec<Instruction>, String> {
    let compute_units =
        apply_compute_unit_margin(compute_units, fee_config.compute_unit_margin_multiplier);

    let mut budget_instructions = Vec::new();
    budget_instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(
        compute_units,
    ));

    match &fee_config.priority_fee {
        PriorityFeeStrategy::Dynamic {
            percentile,
            max_lamports,
        } => {
            let fee =
                calculate_dynamic_priority_fee(client, rpc_config, writable_accounts, *percentile)
                    .await?;
            let clamped_fee = std::cmp::min(fee, *max_lamports);

            if clamped_fee > 0 {
                budget_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(
                    clamped_fee,
                ));
            }
        }
        PriorityFeeStrategy::Exact(lamports) => {
            if *lamports > 0 {
                budget_instructions
                    .push(ComputeBudgetInstruction::set_compute_unit_price(*lamports));
            }
        }
        PriorityFeeStrategy::Disabled => {}
    }

    Ok(budget_instructions)
}

/// Calculate dynamic priority fee based on recent fees
pub(crate) async fn calculate_dynamic_priority_fee(
    client: &RpcClient,
    rpc_config: &RpcConfig,
    writable_accounts: &[Pubkey],
    percentile: Percentile,
) -> Result<u64, String> {
    if rpc_config.supports_priority_fee_percentile {
        get_priority_fee_with_percentile(client, writable_accounts, percentile).await
    } else {
        get_priority_fee_legacy(client, writable_accounts, percentile).await
    }
}

/// Get priority fee using the getRecentPrioritizationFees endpoint with percentile parameter
pub(crate) async fn get_priority_fee_with_percentile(
    client: &RpcClient,
    writable_accounts: &[Pubkey],
    percentile: Percentile,
) -> Result<u64, String> {
    // This is a direct RPC call using reqwest since the Solana client doesn't support
    // the percentile parameter yet
    let rpc_url = client.url();

    let response = reqwest::Client::new()
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getRecentPrioritizationFees",
            "params": [{
                "lockedWritableAccounts": writable_accounts.iter().map(|p| p.to_string()).collect::<Vec<String>>(),
                "percentile": percentile.as_value() * 100
            }]
        }))
        .send()
        .await
        .map_err(|e| format!("RPC Error: {}", e))?;

    #[derive(serde::Deserialize)]
    struct Response {
        result: RpcPrioritizationFee,
    }

    response
        .json::<Response>()
        .await
        .map(|resp| resp.result.prioritization_fee)
        .map_err(|e| format!("Failed to parse prioritization fee response: {}", e))
}

/// Get priority fee using the legacy getRecentPrioritizationFees endpoint
pub(crate) async fn get_priority_fee_legacy(
    client: &RpcClient,
    writable_accounts: &[Pubkey],
    percentile: Percentile,
) -> Result<u64, String> {
    // This uses the built-in method that returns Vec<RpcPrioritizationFee>
    let recent_fees = client
        .get_recent_prioritization_fees(writable_accounts)
        .await
        .map_err(|e| format!("RPC Error: {}", e))?;

    // Filter out zero fees and sort
    let mut non_zero_fees: Vec<u64> = recent_fees
        .iter()
        .filter(|fee| fee.prioritization_fee > 0)
        .map(|fee| fee.prioritization_fee)
        .collect();

    non_zero_fees.sort_unstable();

    if non_zero_fees.is_empty() {
        return Ok(0);
    }

    // Calculate percentile
    let index = (non_zero_fees.len() as f64 * (percentile.as_value() as f64 / 100.0)) as usize;
    let index = std::cmp::min(index, non_zero_fees.len() - 1);

    Ok(non_zero_fees[index])
}

/// Get writable accounts from a list of instructions
pub fn get_writable_accounts(instructions: &[Instruction]) -> Vec<Pubkey> {
    let mut writable = std::collections::HashSet::new();

    for ix in instructions {
        for meta in &ix.accounts {
            if meta.is_writable {
                writable.insert(meta.pubkey);
            }
        }
    }

    writable.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_commitment_config::CommitmentConfig;

    #[test]
    fn simulation_config_preserves_commitment_and_min_context_slot() {
        let config = simulation_config(Some(CommitmentConfig::confirmed()), Some(123));

        assert_eq!(config.commitment, Some(CommitmentConfig::confirmed()));
        assert_eq!(config.min_context_slot, Some(123));
        assert!(!config.sig_verify);
        assert!(config.replace_recent_blockhash);
    }

    #[test]
    fn builder_sets_compute_unit_limit() {
        let config = ComputeConfig::default().with_unit_limit(ComputeUnitLimitStrategy::Exact(321));
        assert!(matches!(
            config.unit_limit,
            ComputeUnitLimitStrategy::Exact(321)
        ));
    }

    #[test]
    fn dynamic_compute_unit_limit_applies_margin_and_clamps() {
        assert_eq!(apply_compute_unit_margin(100_000, 1.1), 110_000);
        assert_eq!(apply_compute_unit_margin(1_350_000, 1.1), 1_400_000);
    }

    #[tokio::test]
    async fn exact_compute_unit_limit_is_used_unchanged() {
        let rpc_client = RpcClient::new("http://127.0.0.1:1".to_string());
        let compute_config =
            ComputeConfig::default().with_unit_limit(ComputeUnitLimitStrategy::Exact(1_400_000));
        let fee_config = FeeConfig::default();
        let rpc_config = RpcConfig::default();

        let instructions = build_compute_budget_instructions(
            &rpc_client,
            &[],
            &Pubkey::default(),
            None,
            &compute_config,
            &fee_config,
            &rpc_config,
            None,
        )
        .await
        .unwrap();

        assert_eq!(instructions.len(), 1);
        assert_eq!(
            u32::from_le_bytes(instructions[0].data[1..5].try_into().unwrap()),
            1_400_000
        );
    }

    #[tokio::test]
    async fn invalid_exact_compute_unit_limits_are_rejected() {
        let rpc_client = RpcClient::new("http://127.0.0.1:1".to_string());
        let too_large_config =
            ComputeConfig::default().with_unit_limit(ComputeUnitLimitStrategy::Exact(1_400_001));
        let fee_config = FeeConfig::default();
        let rpc_config = RpcConfig::default();
        let too_large = build_compute_budget_instructions(
            &rpc_client,
            &[],
            &Pubkey::default(),
            None,
            &too_large_config,
            &fee_config,
            &rpc_config,
            None,
        )
        .await
        .unwrap_err();
        assert!(too_large.contains("between 1 and 1,400,000"));

        let zero_config =
            ComputeConfig::default().with_unit_limit(ComputeUnitLimitStrategy::Exact(0));
        let zero = build_compute_budget_instructions(
            &rpc_client,
            &[],
            &Pubkey::default(),
            None,
            &zero_config,
            &fee_config,
            &rpc_config,
            None,
        )
        .await
        .unwrap_err();
        assert!(zero.contains("between 1 and 1,400,000"));
    }
}
