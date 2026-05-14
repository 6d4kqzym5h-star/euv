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
/// Uses SSE-style long-polling: keeps the HTTP connection open until
/// a reload event is broadcast, then returns "reload" so the client
/// can refresh the page and immediately reconnect.
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
        let mut rx: broadcast::Receiver<()> = state.reload_tx.subscribe();
        let result: String = match rx.recv().await {
            Ok(()) => "reload".to_string(),
            Err(_) => "error".to_string(),
        };
        ctx.get_mut_response()
            .set_header("Content-Type", "text/plain")
            .set_body(&result);
        Status::Continue
    }
}
