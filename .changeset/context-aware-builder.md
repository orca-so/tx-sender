---
"@orca-so/rust-tx-sender": minor
---

Add `build_transaction_with_config_obj_at_context`, which runs the dynamic compute-unit simulation at the rpc client's configured commitment and, when provided, requires the simulation bank to have reached `min_context_slot`. Existing builders keep their current behavior.
