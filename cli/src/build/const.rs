/// Placeholder token used in HTML templates for the JS import path.
///
/// Replaced at runtime with the resolved import path relative to the www directory.
pub(crate) const IMPORT_PATH_PLACEHOLDER: &str = "__IMPORT_PATH__";

/// Placeholder token used in HTML templates for the reload endpoint URL.
///
/// Replaced at runtime with the actual reload route path.
pub(crate) const RELOAD_ROUTE_PLACEHOLDER: &str = "__RELOAD_ROUTE__";

/// The URL path for the reload endpoint.
///
/// Used by the live-reload script in the HTML template and the server route registration.
pub(crate) const RELOAD_ROUTE: &str = "/__euv_reload";

/// The wasm-pack flag indicating a release build.
pub(crate) const RELEASE_FLAG: &str = "--release";

/// The CLI command name for wasm-pack.
pub(crate) const WASM_PACK_COMMAND: &str = "wasm-pack";

/// The wasm-pack subcommand for building.
pub(crate) const WASM_PACK_BUILD_SUBCOMMAND: &str = "build";

/// The wasm-pack argument for specifying the output directory.
pub(crate) const OUT_DIR_ARG: &str = "--out-dir";

/// The wasm-pack argument for specifying the output name.
pub(crate) const OUT_NAME_ARG: &str = "--out-name";

/// The default output subdirectory name for wasm-pack artifacts.
pub(crate) const PKG_DIR_NAME: &str = "pkg";

/// The JavaScript file extension.
pub(crate) const JS_EXTENSION: &str = ".js";

/// The CLI command name for hyperlane-cli.
pub(crate) const HYPERLANE_CLI_COMMAND: &str = "hyperlane-cli";

/// The version flag for CLI tools.
pub(crate) const VERSION_ARG: &str = "--version";

/// The format subcommand for hyperlane-cli.
pub(crate) const FMT_SUBCOMMAND: &str = "fmt";

/// The CLI command name for cargo.
pub(crate) const CARGO_COMMAND: &str = "cargo";

/// The install subcommand for cargo.
pub(crate) const CARGO_INSTALL_SUBCOMMAND: &str = "install";

/// The source directory name within a Cargo project.
pub(crate) const SRC_DIR_NAME: &str = "src";

/// The name of the gitignore file.
pub(crate) const GITIGNORE_FILE_NAME: &str = ".gitignore";

/// The name of the Cargo manifest file.
pub(crate) const CARGO_TOML_FILE_NAME: &str = "Cargo.toml";

/// The TypeScript declaration file extension.
pub(crate) const D_TS_EXTENSION: &str = "d.ts";

/// The npm package manifest file name.
pub(crate) const PACKAGE_JSON_FILE_NAME: &str = "package.json";

/// The npm README file name.
pub(crate) const README_FILE_NAME: &str = "README.md";

/// The npm LICENSE file name.
pub(crate) const LICENSE_FILE_NAME: &str = "LICENSE";

/// The index HTML file name.
pub(crate) const INDEX_HTML_FILE_NAME: &str = "index.html";

/// The relative path prefix used for import path construction.
pub(crate) const RELATIVE_PATH_PREFIX: &str = "./";

/// The path separator used for joining path components.
pub(crate) const PATH_SEPARATOR: &str = "/";

/// The `run` action name used in banner display.
pub(crate) const ACTION_RUN: &str = "run";

/// The `build` action name used in banner display.
pub(crate) const ACTION_BUILD: &str = "build";

/// The `index.html` template for the development profile.
///
/// Includes a live-reload `<script>` block that connects to the
/// `/__euv_reload` endpoint and parses the JSON payload sent by the server.
/// The JSON uses a tagged enum format:
/// - `{"type":"Reload"}` — the client should reload the page.
/// - `{"type":"Error","message":"..."}` — a build error occurred.
///
/// The `__OUT_NAME__` placeholder is replaced with the actual output name at runtime.
pub(crate) const INDEX_HTML_DEV: &str = r#"<!doctype html>
<html>
  <head>
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no"
    />
    <title>euv</title>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script type="module">
    import init, { main } from '__IMPORT_PATH__';
    await init();
    main();
  </script>
  <script>
    (function () {
      async function connect() {
        try {
          const res = await fetch('__RELOAD_ROUTE__');
          const data = await res.json();
          if (data.type === 'Reload') {
            location.reload();
          } else if (data.type === 'Error') {
            console.error('[euv] build error:', data.message);
            setTimeout(connect, 1000);
          } else {
            setTimeout(connect, 1000);
          }
        } catch (e) {
          setTimeout(connect, 2000);
        }
      }
      connect();
    })();
  </script>
</html>
"#;

/// The `index.html` template for the release profile.
///
/// A minimal `index.html` without any live-reload instrumentation.
/// Used when building for release to produce a clean, static entry point.
/// The `__OUT_NAME__` placeholder is replaced with the actual output name at runtime.
pub(crate) const INDEX_HTML_RELEASE: &str = r#"<!doctype html>
<html>
  <head>
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no"
    />
    <title>euv</title>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script type="module">
    import init, { main } from '__IMPORT_PATH__';
    await init();
    main();
  </script>
</html>
"#;
