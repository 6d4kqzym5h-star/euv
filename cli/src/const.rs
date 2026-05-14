/// Default HTML template for development.
pub(crate) const DEFAULT_INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
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
pub(crate) const RELOAD_SCRIPT: &str = r#"
<script>
(function() {
    async function connect() {
        try {
            const res = await fetch('/__euv_reload');
            const text = await res.text();
            if (text === 'reload') {
                location.reload();
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
