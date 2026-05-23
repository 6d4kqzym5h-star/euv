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
