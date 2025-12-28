# Orca TX Sender

A lightweight library for sending transactions to Solana with automatic priority fees, Jito tips, compute unit estimation, and retry logic.

## Features

- **Automatic Priority Fees**: Dynamically calculates optimal priority fees based on recent network conditions
- **Jito Tips**: Optional support for Jito bundles with configurable tip amounts
- **Compute Unit Estimation**: Automatically estimates and sets compute unit limits with configurable margins
- **Retry Logic**: Built-in retry mechanism for failed transactions
- **Address Lookup Tables**: Full support for versioned transactions with ALTs

## Packages

This monorepo contains two packages:

| Package | Description | npm/crates |
|---------|-------------|------------|
| [ts/](./ts) | TypeScript SDK | [@orca-so/tx-sender](https://www.npmjs.com/package/@orca-so/tx-sender) |
| [rust/](./rust) | Rust SDK | [orca_tx_sender](https://crates.io/crates/orca_tx_sender) |

## Documentation

- [TypeScript Documentation](./ts/README.md)
- [Rust Documentation](./rust/README.md)

## Development

### Prerequisites

- Node.js 22+
- Yarn 4.6.0+
- Rust 1.86.0+

### Setup

```bash
# Install dependencies
yarn install

# Build all packages
yarn build

# Run tests
yarn test

# Lint
yarn lint
```

## License

See [LICENSE](./LICENSE)
