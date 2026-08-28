use super::*;

/// Props for the [`euv_markdown`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvMarkdownProps {
    /// The block AST to render.
    #[get(type(copy))]
    pub blocks: &'static [EuvMdBlock],
}
