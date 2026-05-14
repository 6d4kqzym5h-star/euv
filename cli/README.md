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

Start the development server for your euv project:

```shell
# cargo
cargo run -p euv-cli -- --crate-path ./example --www-dir ./www --port 3000
# euv-cli
euv-cli --crate-path ./example --www-dir ./www --port 3000
```

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
