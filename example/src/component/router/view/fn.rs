use crate::*;

/// Resolves the current route to the corresponding page virtual DOM tree.
///
/// Matches the route string against all registered page paths and returns
/// the appropriate page component. Falls back to a 404 page for unknown routes.
///
/// # Arguments
///
/// - `PageRouterProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The virtual DOM tree of the matched page.
#[component]
pub(crate) fn page_router(node: VirtualNode<PageRouterProps>) -> VirtualNode {
    let PageRouterProps { route_signal }: PageRouterProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_router()
            match { route_signal.get().as_str() } {
                "/" | "/about" => {
                    page_about {}
                }
                "/counter" => {
                    page_counter {}
                }
                "/badge" => {
                    page_badge {}
                }
                "/event" => {
                    page_event {}
                }
                "/list" => {
                    page_list {}
                }
                "/observer" => {
                    page_observer {}
                }
                "/conditional" => {
                    page_conditional {}
                }
                "/modal" => {
                    page_modal {}
                }
                "/select" => {
                    page_select {}
                }
                "/async" => {
                    page_async_demo {}
                }
                "/form" => {
                    page_form {}
                }
                "/file-upload" => {
                    page_file_upload {}
                }
                "/timer" => {
                    page_timer {}
                }
                "/animation" => {
                    page_animation {}
                }
                "/browser" => {
                    page_browser {}
                }
                "/lifecycle" => {
                    page_lifecycle {}
                }
                "/keep-alive" => {
                    page_keep_alive {}
                }
                "/component-binding" => {
                    page_component_binding {}
                }
                "/custom-attrs" => {
                    page_custom_attrs {}
                }
                "/dynamic-component" => {
                    page_dynamic_component {}
                }
                "/virtual-list" => {
                    page_virtual_list {}
                }
                "/camera" => {
                    page_camera {}
                }
                "/canvas" => {
                    page_canvas {}
                }
                "/game-2d" => {
                    page_game_2d {}
                }
                "/game-3d" => {
                    page_game_3d {}
                }
                "/sse" => {
                    page_sse {}
                }
                "/websocket" => {
                    page_websocket {}
                }
                _ => {
                    page_not_found {}
                }
            }
        }
    }
}
