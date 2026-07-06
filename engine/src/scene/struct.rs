use crate::*;

/// Manages scene registration, transitions, and lifecycle.
#[derive(Clone, Data, New)]
pub struct SceneManager {
    /// All registered scenes keyed by name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) scenes: HashMap<String, SceneRc>,
    /// The name of the currently active scene, if any.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) current_scene_name: Option<String>,
    /// The name of the next scene to switch to (for deferred transitions).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) pending_scene_name: Option<String>,
}
