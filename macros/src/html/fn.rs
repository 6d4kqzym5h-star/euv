use crate::*;

/// Sets the user-defined component registry for the current thread.
///
/// # Arguments
///
/// - `HashMap<String, String>`- The map of function names to Props type names.
pub(crate) fn set_user_fn_names(names: HashMap<String, String>) {
    unsafe {
        let ptr: *mut MaybeUninit<HashMap<String, String>> = &raw mut USER_FN_NAMES;
        (*ptr).write(names);
    }
}

/// Checks whether a given name corresponds to a user-defined component function.
///
/// # Arguments
///
/// - `&str`- The name to check against the stored component registry.
///
/// # Returns
///
/// - `bool`- `true` if the name exists in the component registry, `false` otherwise.
pub(crate) fn is_user_fn(name: &str) -> bool {
    unsafe {
        let ptr: *const MaybeUninit<HashMap<String, String>> = &raw const USER_FN_NAMES;
        (*ptr).assume_init_ref().contains_key(name)
    }
}

/// Returns the Props type name for a given component function name.
///
/// # Arguments
///
/// - `&str`- The component function name.
///
/// # Returns
///
/// - `Option<&'static str>`- The Props type name if found.
pub(crate) fn get_user_fn_props_type(name: &str) -> Option<&'static str> {
    unsafe {
        let ptr: *const MaybeUninit<HashMap<String, String>> = &raw const USER_FN_NAMES;
        (*ptr).assume_init_ref().get(name).map(|s| s.as_str())
    }
}

/// Parses the input tokens into a euv VNode expression.
///
/// Supports zero, one, or multiple root-level HTML nodes:
/// - `html! {}` → `VirtualNode::Empty`
/// - `html! { div { ... } }` → single `VirtualNode`
/// - `html! { div { ... } span { ... } }` → `VirtualNode::Fragment(vec![...])`
///
/// Before parsing, reads the component registry file to discover which
/// function names are marked as components via `#[component]`. This allows
/// the `html!` macro to distinguish between component function calls and
/// native HTML element tags.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing HTML markup.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing the corresponding virtual node.
pub fn parse_html(input: TokenStream) -> TokenStream {
    let fn_names: HashMap<String, String> = load_component_registry();
    set_user_fn_names(fn_names);
    let tokens: proc_macro2::TokenStream = match parse::<HtmlRoot>(input) {
        Ok(nodes) => nodes.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}

/// Loads the component registry by scanning the project source for `#[component]` annotations.
///
/// Recursively scans `.rs` files under `CARGO_MANIFEST_DIR/src/` and extracts
/// function names and their Props type names from annotated functions.
///
/// # Returns
///
/// - `HashMap<String, String>` - Map of component function names to Props type names.
fn load_component_registry() -> HashMap<String, String> {
    let manifest_dir: Option<String> = env::var(CARGO_MANIFEST_DIR).ok();
    let Some(manifest_dir) = manifest_dir else {
        return HashMap::new();
    };
    let src_dir: PathBuf = PathBuf::from(&manifest_dir).join(SRC_DIR);
    let mut fn_names: HashMap<String, String> = HashMap::new();
    scan_dir_for_components(&src_dir, &mut fn_names);
    fn_names
}

/// Recursively scans a directory for `.rs` files and extracts component function names.
///
/// # Arguments
///
/// - `&PathBuf` - The directory to scan.
/// - `&mut HashMap<String, String>` - The map to populate with discovered component names and Props types.
fn scan_dir_for_components(dir: &PathBuf, fn_names: &mut HashMap<String, String>) {
    let entries: Result<ReadDir, std::io::Error> = read_dir(dir);
    let Ok(entries) = entries else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            scan_dir_for_components(&path, fn_names);
        } else if path
            .extension()
            .is_some_and(|ext: &OsStr| ext == OsStr::new(RUST_FILE_EXTENSION))
        {
            scan_file_for_components(&path, fn_names);
        }
    }
}

/// Parses a single `.rs` file and extracts function names annotated with `#[component]`,
/// along with their first parameter's type name (the Props type).
///
/// # Arguments
///
/// - `&PathBuf` - The file path to parse.
/// - `&mut HashMap<String, String>` - The map to populate with function name → Props type name.
fn scan_file_for_components(path: &PathBuf, fn_names: &mut HashMap<String, String>) {
    let content: String = match read_to_string(path) {
        Ok(data) => data,
        Err(_) => return,
    };
    let file: File = match parse_file(&content) {
        Ok(file) => file,
        Err(_) => return,
    };
    for item in &file.items {
        if let Item::Fn(item_fn) = item {
            let has_component_attr: bool = item_fn
                .attrs
                .iter()
                .any(|attr: &Attribute| attr.path().is_ident(COMPONENT_ATTR));
            if has_component_attr {
                let fn_name: String = item_fn.sig.ident.to_string();
                let props_type: String = extract_props_type_from_fn(item_fn);
                fn_names.insert(fn_name, props_type);
            }
        }
    }
}

/// Extracts the Props type name from the first parameter of a component function.
///
/// Looks for the first parameter's type. If the type is a simple path (e.g., `PrimaryButtonProps`),
/// returns its string representation. Falls back to an empty string if not found.
///
/// # Arguments
///
/// - `&syn::ItemFn` - The function item to extract from.
///
/// # Returns
///
/// - `String` - The Props type name, or empty string if not extractable.
fn extract_props_type_from_fn(item_fn: &syn::ItemFn) -> String {
    let inputs: &syn::punctuated::Punctuated<syn::FnArg, Token![,]> = &item_fn.sig.inputs;
    for input in inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            let ty: &Type = &pat_type.ty;
            if let Type::Path(type_path) = ty
                && let Some(segment) = type_path.path.segments.last()
            {
                return segment.ident.to_string();
            }
        }
    }
    String::new()
}

/// Parses a stream of tokens into a list of HTML child nodes.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream containing HTML child content.
///
/// # Returns
///
/// - `syn::Result<Vec<HtmlNode>>` - The parsed list of HTML child nodes, or a syntax error.
pub(crate) fn parse_html_children(content: ParseStream) -> syn::Result<Vec<HtmlNode>> {
    let mut children: Vec<HtmlNode> = Vec::new();
    while !content.is_empty() {
        if content.peek(LitStr) && content.peek2(Brace) {
            let element: HtmlElement = content.parse()?;
            children.push(HtmlNode::Element(element));
        } else if content.peek(LitStr) {
            let lit: LitStr = content.parse()?;
            children.push(HtmlNode::Text(lit.value()));
        } else if content.peek(Token![if]) {
            let html_if: HtmlIf = content.parse()?;
            children.push(HtmlNode::If(html_if));
        } else if content.peek(Token![match]) {
            let html_match: HtmlMatch = content.parse()?;
            children.push(HtmlNode::Match(html_match));
        } else if content.peek(Token![for]) {
            let html_for: HtmlFor = content.parse()?;
            children.push(HtmlNode::For(html_for));
        } else if content.peek(Brace) {
            let child_content;
            braced!(child_content in content);
            let expr: Expr = child_content.parse()?;
            children.push(HtmlNode::Dynamic(expr));
        } else if (content.peek(Ident) || content.peek(LitStr)) && content.peek2(Colon) {
            break;
        } else if content.peek(Ident) {
            if content.peek2(Brace) {
                let element: HtmlElement = content.parse()?;
                children.push(HtmlNode::Element(element));
            } else {
                let expr: Expr = content.parse()?;
                children.push(HtmlNode::Expr(expr));
            }
        } else {
            return Err(content.error(ERR_UNEXPECTED_TOKEN_IN_HTML));
        }
    }
    Ok(children)
}

/// Parses the body of a match arm after the `=>` token.
///
/// Unlike `parse_html_children` which operates on a braced scope, this function
/// reads directly from the arms content stream and stops when it encounters a
/// top-level comma (indicating the next arm) or the end of the stream.
/// Supports all HTML node types: elements, text, expressions, if, match, for,
/// and braced dynamic expressions.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned after `=>` in a match arm.
///
/// # Returns
///
/// - `syn::Result<Vec<HtmlNode>>` - The parsed list of HTML nodes for the arm body.
pub(crate) fn parse_match_arm_body(content: ParseStream) -> syn::Result<Vec<HtmlNode>> {
    if content.peek(Brace) {
        let child_content;
        braced!(child_content in content);
        parse_html_children(&child_content)
    } else {
        let node: HtmlNode = content.parse()?;
        Ok(vec![node])
    }
}

/// Converts a slice of `HtmlNode` children into a `Vec<proc_macro2::TokenStream>`.
///
/// Shared helper used by both `children_to_node_tokens` and `children_to_tokens`.
#[inline]
fn nodes_to_token_vec(children: &[HtmlNode]) -> Vec<proc_macro2::TokenStream> {
    children
        .iter()
        .map(|child| {
            let mut ts = proc_macro2::TokenStream::new();
            child.to_tokens(&mut ts);
            ts
        })
        .collect()
}

/// Converts a list of `HtmlNode` children into a single `VirtualNode` token stream.
///
/// - 0 children → `VirtualNode::Empty`
/// - 1 child → the child's token stream directly (no Fragment wrapper)
/// - N children → `VirtualNode::Fragment(vec![...])`
///
/// # Arguments
///
/// - `&[HtmlNode]` - The slice of HTML child nodes to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated token stream representing a single `VirtualNode`.
pub(crate) fn children_to_node_tokens(children: &[HtmlNode]) -> proc_macro2::TokenStream {
    match children.len() {
        0 => quote! { ::euv::VirtualNode::Empty },
        1 => {
            let mut ts = proc_macro2::TokenStream::new();
            children[0].to_tokens(&mut ts);
            ts
        }
        _ => {
            let child_tokens = nodes_to_token_vec(children);
            quote! { ::euv::VirtualNode::Fragment(vec![#(#child_tokens), *]) }
        }
    }
}

/// Converts a list of `HtmlNode` children into a `Vec<VirtualNode>` token stream.
///
/// Always produces `vec![...]` format, used by `for` loops where the body
/// is collected and then extended into an accumulator.
///
/// # Arguments
///
/// - `&[HtmlNode]` - The slice of HTML child nodes to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated token stream representing a `Vec<VirtualNode>`.
pub(crate) fn children_to_tokens(children: &[HtmlNode]) -> proc_macro2::TokenStream {
    let child_tokens = nodes_to_token_vec(children);
    quote! { vec![#(#child_tokens), *] }
}

/// Parses a reactive `if {expr} { value } [else if {expr} { value }]* [else { value }]` in attribute value position.
///
/// Unlike `HtmlIf` (which contains HTML child nodes), each branch body here is a Rust expression.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned at the `if` keyword.
///
/// # Returns
///
/// - `syn::Result<HtmlAttrIf>` - The parsed attribute-level reactive conditional.
pub(crate) fn parse_attr_if(content: ParseStream) -> syn::Result<HtmlAttrIf> {
    let mut branches: Vec<(Option<Expr>, Expr)> = Vec::new();
    content.parse::<Token![if]>()?;
    let cond_content;
    braced!(cond_content in content);
    let condition: Expr = cond_content.parse()?;
    let body_content;
    braced!(body_content in content);
    let body: Expr = body_content.parse()?;
    branches.push((Some(condition), body));
    while content.peek(Token![else]) {
        content.parse::<Token![else]>()?;
        if content.peek(Token![if]) {
            content.parse::<Token![if]>()?;
            let cond_content;
            braced!(cond_content in content);
            let condition: Expr = cond_content.parse()?;
            let body_content;
            braced!(body_content in content);
            let body: Expr = body_content.parse()?;
            branches.push((Some(condition), body));
        } else {
            let body_content;
            braced!(body_content in content);
            let body: Expr = body_content.parse()?;
            branches.push((None, body));
            break;
        }
    }
    Ok(HtmlAttrIf { branches })
}

/// Strips outer braces from an `Expr` if it is an `Expr::Block` with a single expression,
/// avoiding Rust `unused_braces` warnings in generated `if` conditions.
///
/// # Arguments
///
/// - `&Expr` - The expression to potentially strip.
///
/// # Returns
///
/// - `&Expr` - The inner expression if the input was a braced single-expression block, otherwise the original.
pub(crate) fn strip_braces_from_expr(expr: &Expr) -> &Expr {
    if let Expr::Block(expr_block) = expr {
        let stmts: &Vec<Stmt> = &expr_block.block.stmts;
        if stmts.len() == 1
            && let Stmt::Expr(inner, None) = &stmts[0]
        {
            return inner;
        }
    }
    expr
}

/// Generates a token stream for an `HtmlAttrIf` as a Rust `if` expression.
///
/// The generated code is used inside a reactive closure so that when signals
/// change, the conditional is re-evaluated.
///
/// # Arguments
///
/// - `&HtmlAttrIf` - The parsed attribute-level reactive conditional.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated `if ... { ... } else if ... { ... } else { ... }` token stream.
pub(crate) fn attr_if_to_tokens(html_attr_if: &HtmlAttrIf) -> proc_macro2::TokenStream {
    let mut if_chain: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    for (branch_index, (condition, body)) in html_attr_if.branches.iter().enumerate() {
        let stripped_body = strip_braces_from_expr(body);
        match (branch_index, condition) {
            (0, Some(cond)) => {
                let stripped_cond = strip_braces_from_expr(cond);
                if_chain.extend(quote! { if #stripped_cond { #stripped_body } });
            }
            (_, Some(cond)) => {
                let stripped_cond = strip_braces_from_expr(cond);
                if_chain.extend(quote! { else if #stripped_cond { #stripped_body } });
            }
            (_, None) => {
                if_chain.extend(quote! { else { #stripped_body } });
            }
        }
    }
    if_chain
}

/// Builds a Rust `if/else if/else` chain token stream from `HtmlIf` branches.
///
/// Each branch body is converted to a `VirtualNode` token stream via `children_to_node_tokens`.
/// Used by `HtmlNode::If` in `ToTokens`.
///
/// # Arguments
///
/// - `&[(Option<Expr>, Vec<HtmlNode>)]` - The branches from an `HtmlIf`.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated if-chain token stream.
pub(crate) fn build_html_if_chain(
    branches: &[(Option<Expr>, Vec<HtmlNode>)],
) -> proc_macro2::TokenStream {
    let mut if_chain = proc_macro2::TokenStream::new();
    for (branch_index, (condition, body)) in branches.iter().enumerate() {
        let body_expr = children_to_node_tokens(body);
        match (branch_index, condition) {
            (0, Some(cond)) => {
                let stripped_cond = strip_braces_from_expr(cond);
                if_chain.extend(quote! { if #stripped_cond { #body_expr } });
            }
            (_, Some(cond)) => {
                let stripped_cond = strip_braces_from_expr(cond);
                if_chain.extend(quote! { else if #stripped_cond { #body_expr } });
            }
            (_, None) => {
                if_chain.extend(quote! { else { #body_expr } });
            }
        }
    }
    if_chain
}

/// Parses the value side of an attribute, handling the special `style:` attribute.
///
/// If the key is `"style"` and the value is a braced expression that looks like
/// a style object (key-value pairs separated by `;`), it is parsed as
/// `HtmlAttrValue::Style`. Otherwise, the value is parsed as a normal expression
/// or a reactive `if` conditional.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned after the ` -` token.
/// - `&str` - The attribute key string (e.g., `"style"`, `"class"`).
///
/// # Returns
///
/// - `syn::Result<HtmlAttrValue>` - The parsed attribute value.
pub(crate) fn parse_attr_value(content: ParseStream, key_str: &str) -> syn::Result<HtmlAttrValue> {
    if content.peek(Token![if]) {
        let html_attr_if: HtmlAttrIf = parse_attr_if(content)?;
        return Ok(HtmlAttrValue::If(html_attr_if));
    }
    if key_str == ATTR_KEY_STYLE && content.peek(Brace) {
        let style_content;
        braced!(style_content in content);
        let is_style_object: bool = style_content.peek(LitStr) || style_content.peek(Ident);
        if is_style_object {
            let mut style_props: Vec<(String, HtmlStylePropValue)> = Vec::new();
            while !style_content.is_empty() {
                let css_key: String = parse_kebab_name(&style_content)?;
                style_content.parse::<Colon>()?;
                let prop_value: HtmlStylePropValue = if style_content.peek(Token![if]) {
                    let html_attr_if: HtmlAttrIf = parse_attr_if(&style_content)?;
                    HtmlStylePropValue::If(html_attr_if)
                } else if style_content.peek(LitStr) {
                    let lit: LitStr = style_content.parse()?;
                    HtmlStylePropValue::Literal(lit.value())
                } else if style_content.peek(Brace) {
                    let expr_content;
                    braced!(expr_content in style_content);
                    if expr_content.peek(Token![if]) {
                        let html_attr_if: HtmlAttrIf = parse_attr_if(&expr_content)?;
                        HtmlStylePropValue::If(html_attr_if)
                    } else {
                        let expr: Expr = expr_content.parse()?;
                        HtmlStylePropValue::Expr(expr)
                    }
                } else {
                    let expr: Expr = style_content.parse()?;
                    HtmlStylePropValue::Expr(expr)
                };
                style_props.push((css_key, prop_value));
                if style_content.peek(Semi) {
                    style_content.parse::<Semi>()?;
                }
            }
            Ok(HtmlAttrValue::Style(style_props))
        } else {
            Ok(HtmlAttrValue::Expr(style_content.parse()?))
        }
    } else {
        Ok(HtmlAttrValue::Expr(content.parse()?))
    }
}

/// Merges attributes with the same key name for `class` and `style`.
///
/// When multiple `class:` or `style:` attributes are declared on the same
/// element, they are combined into a single `HtmlAttrValue::Classes` or
/// `HtmlAttrValue::Styles` entry so that the renderer can merge their
/// values at runtime rather than overwriting.
///
/// Non-mergeable attribute keys keep only the last occurrence.
///
/// # Arguments
///
/// - `Vec<(Ident, HtmlAttrValue)>` - The raw parsed attributes (may contain duplicate keys).
///
/// # Returns
///
/// - `Vec<(Ident, HtmlAttrValue)>` - The merged attributes with at most one `class` and one `style` entry.
pub(crate) fn merge_same_key_attributes(
    attributes: Vec<(Ident, HtmlAttrValue)>,
) -> Vec<(Ident, HtmlAttrValue)> {
    let mut class_values: Vec<HtmlAttrValue> = Vec::new();
    let mut style_values: Vec<HtmlAttrValue> = Vec::new();
    let mut result: Vec<(Ident, HtmlAttrValue)> = Vec::new();
    for (key, value) in attributes {
        match key.to_string().as_str() {
            ATTR_KEY_CLASS => class_values.push(value),
            ATTR_KEY_STYLE => style_values.push(value),
            _ => result.push((key, value)),
        }
    }
    let push_merged = |result: &mut Vec<(Ident, HtmlAttrValue)>,
                       key_str: &str,
                       mut values: Vec<HtmlAttrValue>,
                       wrap: fn(Vec<HtmlAttrValue>) -> HtmlAttrValue| {
        match values.len() {
            0 => {}
            1 => result.push((
                Ident::new(key_str, proc_macro2::Span::call_site()),
                values.remove(0),
            )),
            _ => result.push((
                Ident::new(key_str, proc_macro2::Span::call_site()),
                wrap(values),
            )),
        }
    };
    push_merged(
        &mut result,
        ATTR_KEY_CLASS,
        class_values,
        HtmlAttrValue::Classes,
    );
    push_merged(
        &mut result,
        ATTR_KEY_STYLE,
        style_values,
        HtmlAttrValue::Styles,
    );
    result
}

/// Converts an `HtmlAttrValue` into a token stream that produces an `AttributeValue`.
///
/// This function mirrors the logic in `HtmlElement::ToTokens` for converting
/// attribute values, but always wraps the result as an `AttributeValue` variant
/// suitable for passing to `AttributeValue::merge_class`.
///
/// # Arguments
///
/// - `&HtmlAttrValue` - The attribute value to convert.
/// - `&str` - The attribute key name (used for event detection).
/// - `bool` - Whether this is a component attribute.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Token stream that evaluates to an `AttributeValue`.
pub(crate) fn attr_value_to_attribute_value_tokens(
    value: &HtmlAttrValue,
    key_str: &str,
    is_component: bool,
) -> proc_macro2::TokenStream {
    match value {
        HtmlAttrValue::Expr(expr) => {
            if let Some(event_name_str) = key_str.strip_prefix(EVENT_ATTR_PREFIX) {
                if is_component {
                    let callback_name: String = key_str.replace(CHAR_UNDERSCORE, STR_HYPHEN);
                    quote! {
                        ::euv::AttrValueAdapter::new(#expr).into_callback_attribute_value_with_name(#callback_name)
                    }
                } else {
                    quote! {
                        ::euv::EventAdapter::new(#expr).into_attribute(#event_name_str)
                    }
                }
            } else if key_str == ATTR_KEY_CHILDREN {
                quote! { ::euv::AttributeValue::Dynamic(Box::new(#expr)) }
            } else {
                quote! {
                    ::euv::AttrValueAdapter::new(#expr).into_reactive_attribute_value()
                }
            }
        }
        HtmlAttrValue::If(_) => {
            quote! { #value }
        }
        HtmlAttrValue::Style(props) => {
            let has_if: bool = props
                .iter()
                .any(|(_, style_value)| matches!(style_value, HtmlStylePropValue::If(_)));
            if has_if {
                quote! { #value }
            } else {
                quote! { ::euv::AttributeValue::Text(#value) }
            }
        }
        HtmlAttrValue::Classes(_) | HtmlAttrValue::Styles(_) => {
            quote! { #value }
        }
    }
}

/// Converts a style-related `HtmlAttrValue` into a token stream that produces
/// an `AttributeValue`.
///
/// Style values are wrapped in `AttributeValue::Text(...)` for static strings,
/// or kept as `AttributeValue::Signal(...)` for reactive style attributes.
///
/// # Arguments
///
/// - `&HtmlAttrValue` - The style attribute value to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Token stream that evaluates to an `AttributeValue`.
pub(crate) fn style_value_to_attribute_value_tokens(
    value: &HtmlAttrValue,
) -> proc_macro2::TokenStream {
    match value {
        HtmlAttrValue::Style(props) => {
            let has_if: bool = props
                .iter()
                .any(|(_, style_value)| matches!(style_value, HtmlStylePropValue::If(_)));
            if has_if {
                quote! { #value }
            } else {
                quote! { ::euv::AttributeValue::Text(#value) }
            }
        }
        HtmlAttrValue::If(_) => {
            quote! { #value }
        }
        HtmlAttrValue::Expr(expr) => {
            quote! { ::euv::AttributeValue::Text(#expr.to_string()) }
        }
        HtmlAttrValue::Classes(_) | HtmlAttrValue::Styles(_) => {
            quote! { #value }
        }
    }
}
