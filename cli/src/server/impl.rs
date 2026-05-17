use crate::*;

/// Implementation of server hook for request middleware.
impl ServerHook for RequestMiddleware {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CACHE_CONTROL, NO_CACHE_NO_STORE_MUST_REVALIDATE)
            .set_header(PRAGMA, NO_CACHE)
            .set_header(EXPIRES, EXPIRES_DISABLED);
        Status::Continue
    }
}

/// Implementation of server hook for response middleware.
impl ServerHook for ResponseMiddleware {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let response: Vec<u8> = ctx.get_mut_response().build();
        if stream.try_send(&response).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Continue
    }
}

/// Implementation of server hook for index route.
///
/// When the request targets `index.html`, returns the in-memory HTML
/// that has the live-reload script injected. For all other files the
/// handler reads the content from disk.
impl ServerHook for IndexRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let path_opt: Option<String> = ctx.try_get_route_param("path");
        let path: String = path_opt.unwrap_or_default();
        if path.contains("..") || path.starts_with("/") || path.starts_with("\\") {
            ctx.get_mut_response().set_status_code(403);
            return Status::Continue;
        }
        let state: Arc<AppState> = match get_global_state() {
            Some(state) => state,
            None => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        if path.is_empty() || path == "index.html" {
            let html: String = state.html_content.read().await.clone();
            ctx.get_mut_response()
                .set_body(&html)
                .set_header(CONTENT_TYPE, TEXT_HTML);
            return Status::Continue;
        }
        let www_absolute: PathBuf = if state.args.www_dir.is_absolute() {
            state.args.www_dir.clone()
        } else {
            state.args.crate_path.join(&state.args.www_dir)
        };
        let www_absolute: PathBuf = resolve_www_dir(&www_absolute);
        let file_path: PathBuf = www_absolute.join(&path);
        let canonical_path: PathBuf = match fs::canonicalize(&file_path).await {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
                return Status::Continue;
            }
        };
        let base_canonical: PathBuf = match fs::canonicalize(&www_absolute).await {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        if !canonical_path.starts_with(&base_canonical) {
            ctx.get_mut_response().set_status_code(403);
            return Status::Continue;
        }
        match fs::read(&file_path).await {
            Ok(content) => {
                let extension: String = FileExtension::get_extension_name(&path);
                let content_type: &'static str =
                    FileExtension::parse(&extension).get_content_type();
                ctx.get_mut_response()
                    .set_body(&content)
                    .set_header(CONTENT_TYPE, content_type);
            }
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
            }
        }
        Status::Continue
    }
}

/// Implementation of server hook for reload route.
///
/// Uses long-polling: holds the connection open until a reload event
/// is broadcast, then returns a single JSON response so the client
/// can distinguish between a successful rebuild and an error.
impl ServerHook for ReloadRoute {
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let state: Arc<AppState> = match get_global_state() {
            Some(state) => state,
            None => {
                ctx.get_mut_response()
                    .set_status_code(500)
                    .set_body("server not ready");
                return Status::Continue;
            }
        };
        let mut rx: broadcast::Receiver<ReloadEvent> = state.reload_tx.subscribe();
        let event: ReloadEvent = match rx.recv().await {
            Ok(event) => event,
            Err(_) => {
                ctx.get_mut_response().set_status_code(503);
                return Status::Continue;
            }
        };
        let body: String = match serde_json::to_string(&event) {
            Ok(json) => json,
            Err(_) => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        ctx.get_mut_response().set_body(&body);
        Status::Continue
    }
}
