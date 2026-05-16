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

> The official CLI tool for the euv UI framework, providing a development server with hot reload and wasm-pack integration.

## Installation

```shell
cargo install euv-cli
```

## Usage

The CLI supports two profiles (`dev` / `release`) and two modes (`run` / `build`):

```shell
# Development mode: build with debug assertions and start dev server with hot-reload
euv-cli dev run
euv-cli dev run --crate-path ./example --port 8080

# Development mode: build only, do not start the server
euv-cli dev build
euv-cli dev build --crate-path ./example

# Release mode: build with optimizations and start the server
euv-cli release run
euv-cli release run --crate-path ./example

# Release mode: build only
euv-cli release build
euv-cli release build --crate-path ./example

# Or via cargo
cargo run -p euv-cli -- dev run --crate-path ./example --port 3000
cargo run -p euv-cli -- release build --crate-path ./example
```

### Options

| Option         | Short | Default   | Description                                             |
| -------------- | ----- | --------- | ------------------------------------------------------- |
| `--crate-path` | `-c`  | `.`       | Path to the Rust crate containing the WASM application  |
| `--www-dir`    | `-w`  | `www`     | Directory to serve static files from                    |
| `--port`       | `-p`  | `3000`    | Port for the development server                         |
| `--out-dir`    | `-o`  | `www/pkg` | Output directory for wasm-pack (relative to crate_path) |

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
