# AGENTS.md

## Cursor Cloud specific instructions

`svgo-rs` is a Rust SVG optimizer exposed to Node.js via NAPI-RS. The public API is a
single `optimize(inputXml: string): string` function (see `README.md`). It is a library,
not a service — there is nothing to "serve"; you exercise it by importing the built addon.

### Toolchain
- The NAPI crates require **rustc 1.88+**. The base image may default to an older Rust
  (e.g. 1.83); the update script installs and defaults to the `stable` toolchain, which
  includes `clippy` and `rustfmt`.
- Node 22 and Yarn 4 (via Corepack) are used. Dependencies install with `yarn install`.

### Building the native addon (required before tests / running)
- The native addon (`svgo-rs.<platform>.node`) is **not** committed and must be built
  before `yarn test` or importing `index.js` will work.
- Dev build: `yarn build:debug` (`napi build --platform`, ~10s). Release: `yarn build`.
- Gotcha: `yarn build` / `yarn build:debug` **regenerates** `index.js` and
  `wasi-worker-browser.mjs` from NAPI-CLI templates. The regenerated
  `wasi-worker-browser.mjs` drops the committed `/* biome-ignore ... noConsole */`
  comment, which makes `yarn lint` fail with a `noConsole` error. These regenerated files
  are build artifacts — do not commit them. Revert them before linting/committing:
  `git checkout -- index.js wasi-worker-browser.mjs`. On a clean checkout `yarn lint`
  passes.

### Common commands (mirrors CI in `.github/workflows`)
- Lint (JS/TS): `yarn lint` (Biome)
- Rust format check: `cargo fmt -- --check`
- Rust lint: `cargo clippy` (currently emits pre-existing warnings; CI does not treat
  warnings as errors)
- Tests: `yarn test` (Vitest — needs the built addon)
- Benchmark: `yarn bench`
