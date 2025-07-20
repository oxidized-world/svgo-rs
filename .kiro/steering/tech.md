# Technology Stack

## Core Technologies
- **Rust**: Primary implementation language for performance-critical SVG processing
- **NAPI-RS**: Node.js native addon framework for Rust bindings
- **Node.js**: JavaScript runtime integration and package distribution

## Key Dependencies
- `quick-xml`: Fast XML parsing and manipulation
- `bumpalo`: Arena-based memory allocation for efficient AST management
- `regex`: Pattern matching for SVG optimization rules
- `napi` + `napi-derive`: Node.js binding generation

## Build System
- **Cargo**: Rust package manager and build tool
- **napi-build**: Build script integration for Node.js bindings
- **Yarn**: Node.js package manager (v4.9.2)

## Development Tools
- **Biome**: JavaScript/TypeScript linting and formatting
- **Taplo**: TOML formatting
- **Husky**: Git hooks for pre-commit checks
- **Vitest**: Testing framework

## Common Commands

### Building
```bash
# Build release version with platform-specific optimizations
yarn build

# Build debug version
yarn build:debug

# Build for all target platforms
napi build --platform --release
```

### Development
```bash
# Run tests
yarn test

# Run benchmarks
yarn bench

# Format code
yarn format        # Format both Rust and TOML
cargo fmt         # Format Rust only
yarn format:toml  # Format TOML only

# Lint JavaScript/TypeScript
yarn lint
```

### Release
```bash
# Prepare for publishing
yarn prepublishOnly

# Generate artifacts for distribution
yarn artifacts
```

## Build Configuration
- Release builds use LTO (Link Time Optimization) and symbol stripping
- Supports 14+ target platforms including WASM
- Uses cdylib crate type for dynamic library generation
