/// The global bridge object key on `window`.
pub(crate) const BRIDGE_GLOBAL_KEY: &str = "bridge";

/// The `core` property key on the bridge global object.
pub(crate) const CORE_KEY: &str = "core";

/// The `invoke` function key on the bridge core module.
pub(crate) const INVOKE_KEY: &str = "invoke";

/// The argument key for specifying a bridge group name.
pub(crate) const BRIDGE_GROUP_KEY: &str = "group";

/// The invoke command name for resolving bridge group permissions.
pub(crate) const INVOKE_RESOLVE_BRIDGE_GROUP_PERMISSIONS: &str = "resolve_bridge_group_permissions";

/// The bridge group name for querying all permissions.
pub(crate) const BRIDGE_GROUP_ALL: &str = "all";

/// The URL for checking the latest euv crate documentation status.
pub(crate) const DOCS_STATUS_URL: &str = "https://docs.rs/crate/euv/latest/status.json";

/// The invoke command name for updating the local cache via bridge.
pub(crate) const INVOKE_UPDATE_CACHE: &str = "update_cache";
