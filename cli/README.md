<center>

## euv-cli

[![](https://img.shields.io/crates/v/euv-cli.svg)](https://crates.io/crates/euv-cli)
[![](https://img.shields.io/crates/d/euv-cli.svg)](https://img.shields.io/crates/d/euv-cli.svg)
[![](https://docs.rs/euv-cli/badge.svg)](https://docs.rs/euv-cli)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/euv-cli.svg)](../LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/euv/)

[Api Docs](https://docs.rs/euv-cli/latest/)

> The official CLI tool for the euv UI framework, providing run/build modes with hot reload and wasm-pack integration.

## Installation

```shell
cargo install euv-cli
```

## Usage

The CLI uses the pattern `euv <action>` where:

- **Action**: `run` (build + start server) or `build` (build only)

All `wasm-pack build` arguments are transparently forwarded after `--`.

### Run (build + dev server)

```shell
# Build and start dev server with hot-reload
euv run -- --target web --out-name euv

# Build and start dev server with custom crate path and port
euv run --crate-path ./example --port 3000 -- --target web --out-name euv

# Release build with dev server
euv run --crate-path ./example -- --release --target web --out-name euv

# Use a custom static assets directory instead of the default "www"
euv run --www-dir public -- --target web --out-name euv
```

### Build (build only)

```shell
# Dev build
euv build -- --target web --out-name euv

# Release build
euv build -- --release --target web --out-name euv

# Build with custom crate path and static directory
euv build --crate-path ./example --www-dir public -- --target web --out-name euv
```

### Via Cargo

```shell
cargo run -p euv-cli -- run --crate-path ./example --port 3000 -- --target web --out-dir www/pkg --out-name euv
cargo run -p euv-cli -- build --crate-path ./example -- --release --target web --out-dir www/pkg --out-name euv

# Custom static directory
cargo run -p euv-cli -- run --crate-path ./example --www-dir public -- --target web --out-name euv
```

### euv Options

| Option         | Short | Default | Description                                                                  |
| -------------- | ----- | ------- | ---------------------------------------------------------------------------- |
| `--crate-path` | `-c`  | `.`     | Path to the Rust crate containing the WASM application                       |
| `--port`       | `-p`  | `3000`  | Port for the development server                                              |
| `--www-dir`    |       | `www`   | Directory name for static assets and generated HTML (relative to crate-path) |

### How `--www-dir` Works

The `--www-dir` option controls where the CLI looks for and generates static assets:

- **`index.html`** is generated inside `{crate-path}/{www-dir}/`
- **WASM artifacts** are placed in `{crate-path}/{www-dir}/pkg/` by default (unless overridden by `--out-dir`)
- **Dev server** serves files under the `/{www-dir}/` route prefix
- **JS import path** in `index.html` is automatically computed relative to the www directory

For example, with `--www-dir public`:

```
example/
├── public/           ← static assets directory
│   ├── index.html    ← generated HTML
│   └── pkg/          ← WASM build output
│       ├── euv.js
│       └── euv_bg.wasm
```

### wasm-pack Options (transparent passthrough after `--`)

All `wasm-pack build` flags are forwarded as-is. Common options:

| Option              | Description                                                |
| ------------------- | ---------------------------------------------------------- |
| `--target`          | Target environment: bundler, nodejs, web, no-modules, deno |
| `--out-dir`         | Output directory with a relative path                      |
| `--out-name`        | Output file names, defaults to package name                |
| `--dev`             | Create a development build                                 |
| `--release`         | Create a release build                                     |
| `--profiling`       | Create a profiling build                                   |
| `--scope`           | The npm scope to use in package.json                       |
| `--mode`            | Steps to be run: no-install, normal, force                 |
| `--no-typescript`   | Disable TypeScript declaration file generation             |
| `--weak-refs`       | Enable usage of the JS weak references proposal            |
| `--reference-types` | Enable usage of WebAssembly reference types                |
| `--no-pack`         | Do not generate a package.json                             |
| `--no-opt`          | Skip optimization with wasm-opt                            |

See `wasm-pack build --help` for the full list of options.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
