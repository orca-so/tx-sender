---
"@orca-so/rust-tx-sender": patch
---

Preserve exact compute unit limits without applying the configured margin. Dynamically estimated limits continue to apply the margin and are capped at 1,400,000 compute units, while invalid exact limits now return an error.
