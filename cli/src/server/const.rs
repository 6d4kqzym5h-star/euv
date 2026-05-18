/// Default HTML template for development.
pub(crate) const DEFAULT_INDEX_HTML: &str = r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>euv app</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module">
      import init, { main } from './pkg/euv_example.js';
      await init();
      main();
    </script>
  </body>
</html>
"#;

/// Live-reload script injected into the served HTML.
///
/// Connects to the `/__euv_reload` endpoint and parses the JSON
/// payload sent by the server. The JSON uses a tagged enum format:
/// - `{"type":"Reload"}` — the client should reload the page.
/// - `{"type":"Error","message":"..."}` — a build error occurred.
pub(crate) const RELOAD_SCRIPT: &str = r#"
<script>
  (function () {
    async function connect() {
      try {
        const res = await fetch('/__euv_reload');
        const data = await res.json();
        if (data.type === 'Reload') {
          location.reload();
        } else if (data.type === 'Error') {
          console.error('[euv] Build error:', data.message);
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
"#;
