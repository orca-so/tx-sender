---
"@orca-so/rust-tx-sender": major
---

Make `BuildTransactionConfig`, `FeeConfig`, `RpcConfig`, and `ComputeConfig` non-exhaustive and add builder-style setters, including `with_min_context_slot`. Dynamic compute-unit simulation now uses the rpc client's configured commitment and can require a bank at least as recent as the state used to build the transaction.

Breaking: downstream crates can no longer construct these config types with struct literals. Start from each config's `default()` value and use its `with_*` setters instead. `RpcConfig::new` remains available when chain detection is required.
