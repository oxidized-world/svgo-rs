# Project Structure

## Root Directory
- `Cargo.toml` - Rust package configuration and dependencies
- `package.json` - Node.js package configuration and scripts
- `build.rs` - Rust build script for NAPI integration
- `index.js` / `index.d.ts` - Node.js entry points and TypeScript definitions

## Source Code (`src/`)
- `lib.rs` - Main library entry point with NAPI exports and plugin registration
- `parser.rs` - XML/SVG parsing logic using quick-xml
- `optimizer.rs` - Core optimization engine with plugin system and AST traversal
- `plugins/` - Individual optimization plugins

## Plugin Architecture (`src/plugins/`)
Each plugin follows a consistent pattern:
- `mod.rs` - Plugin module declarations
- `{plugin_name}.rs` - Individual plugin implementations

### Plugin Structure
- Configuration struct: `{PluginName}Config`
- Implementation struct: `{PluginName}Plugin<'a>`
- Implements `Plugin<'a>` trait with lifecycle hooks:
  - `root_enter/exit` - Process entire document
  - `element_enter/exit` - Process XML elements
  - `text/comment/doctype_enter/exit` - Process specific node types
  - Return `VisitAction::Keep` or `VisitAction::Remove`

## Memory Management
- Uses `bumpalo::Bump` arena allocator for efficient AST memory management
- All string slices and collections allocated in arena with lifetime `'a`
- Avoids heap allocations during optimization passes

## Testing & Benchmarks
- `__test__/` - Test suite with TypeScript configuration
- `benchmark/` - Performance benchmarks comparing against JavaScript SVGO

## Build Artifacts
- `target/` - Rust compilation artifacts
- `*.node` files - Platform-specific native binaries
- `*.wasi.*` files - WebAssembly builds

## Configuration Files
- `.cargo/config.toml` - Cargo build configuration
- `rustfmt.toml` - Rust code formatting rules
- `biome.json` - JavaScript/TypeScript linting configuration
- `.taplo.toml` - TOML formatting configuration
