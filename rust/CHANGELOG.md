# @orca-so/rust-tx-sender

## 4.0.1

### Patch Changes

- [#30](https://github.com/orca-so/tx-sender/pull/30) [`8d237f1`](https://github.com/orca-so/tx-sender/commit/8d237f1ead9bd69169330268f203cd8db85056ab) Thanks [@jshiohaha](https://github.com/jshiohaha)! - Preserve exact compute unit limits without applying the configured margin. Dynamically estimated limits continue to apply the margin and are capped at 1,400,000 compute units, while invalid exact limits now return an error.

- [#28](https://github.com/orca-so/tx-sender/pull/28) [`6458501`](https://github.com/orca-so/tx-sender/commit/645850145b80fc70c81add6afa7845dd48d3c933) Thanks [@jshiohaha](https://github.com/jshiohaha)! - change license email

## 4.0.0

### Major Changes

- [#27](https://github.com/orca-so/tx-sender/pull/27) [`da38381`](https://github.com/orca-so/tx-sender/commit/da38381e7fba6dbf69782af1d3c880597a122b0f) Thanks [@defnotzed](https://github.com/defnotzed)! - Make `BuildTransactionConfig`, `FeeConfig`, `RpcConfig`, and `ComputeConfig` non-exhaustive and add builder-style setters, including `with_min_context_slot`. Dynamic compute-unit simulation now uses the rpc client's configured commitment and can require a bank at least as recent as the state used to build the transaction.

  Breaking: downstream crates can no longer construct these config types with struct literals. Start from each config's `default()` value and use its `with_*` setters instead. `RpcConfig::new` remains available when chain detection is required.

## 3.0.1

### Patch Changes

- [#26](https://github.com/orca-so/tx-sender/pull/26) [`03a8309`](https://github.com/orca-so/tx-sender/commit/03a8309e714c335b96ae55aeaa55d283e4a35140) Thanks [@defnotzed](https://github.com/defnotzed)! - Expose simulation logs in transaction simulation failure errors.

## 3.0.0

### Major Changes

- [#1162](https://github.com/orca-so/whirlpools/pull/1162) [`14c5655`](https://github.com/orca-so/whirlpools/commit/14c5655b664b1a7484b5a630ed65c7b13965ab5e) Thanks [@jshiohaha](https://github.com/jshiohaha)! - Update solana rust dependencies from v2 to v3. Fix some compilation warnings.

## 2.1.0

### Minor Changes

- [#1026](https://github.com/orca-so/whirlpools/pull/1026) [`735d4ff`](https://github.com/orca-so/whirlpools/commit/735d4ff05bd5197821352169f6961c16fefb2405) Thanks [@jshiohaha](https://github.com/jshiohaha)! - Provide the ability to skip simulation for compute units, set correct number of default signatures when creating a Versioned Transaction

## 2.0.1

### Patch Changes

- [#944](https://github.com/orca-so/whirlpools/pull/944) [`6d601bb`](https://github.com/orca-so/whirlpools/commit/6d601bb4689e5c8e67086eb6e792d6c12e41fab1) Thanks [@jshiohaha](https://github.com/jshiohaha)! - Add instruction with max compute unit limit for simulation if none are provided

## 2.0.0

### Major Changes

- [#929](https://github.com/orca-so/whirlpools/pull/929) [`62ba4ca`](https://github.com/orca-so/whirlpools/commit/62ba4ca4e1eba67898865b2c1ccc78af6e1f860a) Thanks [@jshiohaha](https://github.com/jshiohaha)! - BREAKING: Changed build_transaction to accept the transaction payer instead of a list of signers because the caller might not have access to all signers when building a transaction. Under the hood, we rely on the `num_required_signers` in a compiled message to determine how many signatures to include when creating a VersionedTransaction

## 1.0.1

### Patch Changes

- [#924](https://github.com/orca-so/whirlpools/pull/924) [`661acea`](https://github.com/orca-so/whirlpools/commit/661acea57e54627753d9904c05f2784882801eee) Thanks [@wjthieme](https://github.com/wjthieme)! - Overload main functions to make tx-sender usable in a multi-thread environment

## 1.0.0

### Major Changes

- [#921](https://github.com/orca-so/whirlpools/pull/921) [`356d585`](https://github.com/orca-so/whirlpools/commit/356d5858fa45e6a13dd6d2b9f032550357748ef8) Thanks [@calintje](https://github.com/calintje)! - BREAKING: Changed build_transaction to accept signers array instead of single payer. Fixed transaction signature mismatch in compute unit estimation that caused "accounts offsets" errors.
