use crate::*;

/// A tags demo page showcasing all standard HTML5 elements.
///
/// Organized into semantic sections: Document Metadata, Sectioning, Heading,
/// Flow Content, Phrasing Content, Embedded Content, Interactive, Table,
/// Form, and Deprecated elements.
///
/// # Returns
///
/// - `VirtualNode` - The tags demo page virtual DOM tree.
#[component]
pub(crate) fn page_tags(node: VirtualNode<PageTagsProps>) -> VirtualNode {
    let PageTagsProps = node.try_get_props().unwrap_or_default();
    let dialog_open: Signal<bool> = use_signal(|| false);
    let on_form_submit = move |event: Event| {
        event.prevent_default();
    };
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🏷️"
                title: TAGS_PAGE_TITLE
                subtitle: TAGS_PAGE_SUBTITLE
            }
            // ═══════════════════════════════════════════════════════════════
            // Sectioning Content
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_SECTIONING_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<header>"
                    }
                    p {
                        class: c_tag_desc()
                        "Introductory content or navigational aids for the nearest ancestor sectioning content."
                    }
                    header {
                        class: c_tag_demo_box()
                        h5 {
                            class: c_tag_demo_heading()
                            "Header Example"
                        }
                        p {
                            class: c_tag_demo_text()
                            "This is a header element."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<nav>"
                    }
                    p {
                        class: c_tag_desc()
                        "A section of navigation links."
                    }
                    nav {
                        class: c_tag_demo_box()
                        a {
                            class: c_tag_nav_link()
                            href: TAGS_HREF_PLACEHOLDER
                            target: "_blank"
                            "Home"
                        }
                        a {
                            class: c_tag_nav_link()
                            href: TAGS_HREF_PLACEHOLDER
                            target: "_blank"
                            "About"
                        }
                        a {
                            class: c_tag_nav_link()
                            href: TAGS_HREF_PLACEHOLDER
                            target: "_blank"
                            "Contact"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<main>"
                    }
                    p {
                        class: c_tag_desc()
                        "The dominant content of the body. Only one per document."
                    }
                    div {
                        class: c_tag_demo_box()
                        p {
                            class: c_tag_demo_text()
                            "(main element shown in the app shell, not nested here)"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<section>"
                    }
                    p {
                        class: c_tag_desc()
                        "A thematic grouping of content, typically with a heading."
                    }
                    section {
                        class: c_tag_demo_box()
                        h5 {
                            class: c_tag_demo_heading()
                            "Section Title"
                        }
                        p {
                            class: c_tag_demo_text()
                            "Content inside a section element."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<article>"
                    }
                    p {
                        class: c_tag_desc()
                        "A self-contained composition intended for independent distribution."
                    }
                    article {
                        class: c_tag_demo_box()
                        h5 {
                            class: c_tag_demo_heading()
                            "Article Title"
                        }
                        p {
                            class: c_tag_demo_text()
                            "This is an independent article."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<aside>"
                    }
                    p {
                        class: c_tag_desc()
                        "Content tangentially related to the surrounding content."
                    }
                    aside {
                        class: c_tag_demo_box()
                        p {
                            class: c_tag_demo_text()
                            "This is a sidebar or aside content."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<footer>"
                    }
                    p {
                        class: c_tag_desc()
                        "Footer for the nearest ancestor sectioning content."
                    }
                    footer {
                        class: c_tag_demo_box()
                        p {
                            class: c_tag_demo_text()
                            "Copyright 2026. All rights reserved."
                        }
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // Heading Content
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_HEADING_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<h1> - <h6>"
                    }
                    p {
                        class: c_tag_desc()
                        "Section headings, h1 is the highest level, h6 the lowest."
                    }
                    div {
                        class: c_tag_demo_box()
                        h1 {
                            class: c_heading_h1()
                            "Heading 1"
                        }
                        h2 {
                            class: c_heading_h2()
                            "Heading 2"
                        }
                        h3 {
                            class: c_heading_h3()
                            "Heading 3"
                        }
                        h4 {
                            class: c_heading_h4()
                            "Heading 4"
                        }
                        h5 {
                            class: c_heading_h5()
                            "Heading 5"
                        }
                        h6 {
                            class: c_heading_h6()
                            "Heading 6"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<hgroup>"
                    }
                    p {
                        class: c_tag_desc()
                        "A heading and related content (subtitle, tagline)."
                    }
                    hgroup {
                        class: c_tag_demo_box()
                        h3 {
                            class: c_heading_h3()
                            "Main Heading"
                        }
                        p {
                            class: c_tag_demo_muted()
                            "A subtitle or tagline for the heading"
                        }
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // Block-level / Flow Content
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_FLOW_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<p>"
                    }
                    p {
                        class: c_tag_desc()
                        "A paragraph of text."
                    }
                    p {
                        class: c_tag_demo_box()
                        "This is a paragraph element. It represents a block of text with some margin."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<hr>"
                    }
                    p {
                        class: c_tag_desc()
                        "A thematic break between paragraph-level elements."
                    }
                    div {
                        class: c_tag_demo_box()
                        p {
                            class: c_tag_demo_text()
                            "Content above the rule"
                        }
                        hr {
                            class: c_tag_hr()
                        }
                        p {
                            class: c_tag_demo_text()
                            "Content below the rule"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<pre>"
                    }
                    p {
                        class: c_tag_desc()
                        "Preformatted text preserving whitespace."
                    }
                    pre {
                        class: c_tag_demo_pre()
                        "fn main() {\n    println!(\"Hello, world!\");\n}"
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<blockquote>"
                    }
                    p {
                        class: c_tag_desc()
                        "A section quoted from another source."
                    }
                    blockquote {
                        class: c_tag_demo_blockquote()
                        "The only way to do great work is to love what you do."
                        footer {
                            class: c_tag_demo_muted()
                            "— Steve Jobs"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<figure> + <figcaption>"
                    }
                    p {
                        class: c_tag_desc()
                        "Self-contained content with an optional caption."
                    }
                    figure {
                        class: c_tag_demo_box()
                        div {
                            class: c_tag_figure_placeholder()
                            "[Image Placeholder]"
                        }
                        figcaption {
                            class: c_tag_demo_muted()
                            "Fig.1 — A description of the image above."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<div>"
                    }
                    p {
                        class: c_tag_desc()
                        "A generic container with no semantic meaning."
                    }
                    div {
                        class: c_tag_demo_box()
                        "A generic div container element."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<address>"
                    }
                    p {
                        class: c_tag_desc()
                        "Contact information for a person or organization."
                    }
                    address {
                        class: c_tag_demo_box()
                        "Euv Dev" br {}
                        "123 Framework Lane" br {}
                        a {
                            class: c_tag_nav_link()
                            href: TAGS_EMAIL_ADDRESS
                            target: "_blank"
                            TAGS_EMAIL_DISPLAY
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<details> + <summary>"
                    }
                    p {
                        class: c_tag_desc()
                        "A disclosure widget with a summary."
                    }
                    details {
                        class: c_tag_demo_details()
                        summary {
                            class: c_tag_demo_summary()
                            "Click to expand details"
                        }
                        p {
                            class: c_tag_demo_text()
                            "This content is hidden by default and shown when the summary is clicked."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<dialog>"
                    }
                    p {
                        class: c_tag_desc()
                        "A dialog box or other interactive component."
                    }
                    div {
                        class: c_tag_demo_box()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Toggle Dialog"
                            onclick: use_section_toggle(dialog_open)
                            if { dialog_open.get() } {
                                TAGS_DIALOG_CLOSE
                            } else {
                                TAGS_DIALOG_OPEN
                            }
                        }
                        dialog {
                            class: c_tag_demo_dialog()
                            open: dialog_open.get()
                            "This is a dialog element toggled by a button."
                        }
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // Inline / Phrasing Content
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_PHRASING_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<a>"
                    }
                    p {
                        class: c_tag_desc()
                        "A hyperlink to another location."
                    }
                    div {
                        class: c_tag_demo_box()
                        a {
                            class: c_tag_nav_link()
                            href: TAGS_HREF_PLACEHOLDER
                            target: "_blank"
                            "This is a link"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<span>"
                    }
                    p {
                        class: c_tag_desc()
                        "A generic inline container."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is a "
                        span {
                            class: c_tag_demo_highlight()
                            "highlighted span"
                        }
                        " inside text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<strong>"
                    }
                    p {
                        class: c_tag_desc()
                        "Strong importance, seriousness, or urgency."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is "
                        strong {
                            "strongly important"
                        }
                        " text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<em>"
                    }
                    p {
                        class: c_tag_desc()
                        "Stress emphasis of content."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is "
                        em {
                            "emphasized"
                        }
                        " text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<b>"
                    }
                    p {
                        class: c_tag_desc()
                        "Attention-drawing text without extra importance."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is "
                        b {
                            "bold"
                        }
                        " text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<i>"
                    }
                    p {
                        class: c_tag_desc()
                        "Text in an alternate voice or mood."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is "
                        i {
                            "italic"
                        }
                        " text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<u>"
                    }
                    p {
                        class: c_tag_desc()
                        "Unarticulated annotation (underlined)."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is "
                        u {
                            "underlined"
                        }
                        " text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<s>"
                    }
                    p {
                        class: c_tag_desc()
                        "Content no longer accurate or relevant (strikethrough)."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is "
                        s {
                            "no longer relevant"
                        }
                        " text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<small>"
                    }
                    p {
                        class: c_tag_desc()
                        "Side comments or small print."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Regular text "
                        small {
                            "with small print"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<mark>"
                    }
                    p {
                        class: c_tag_desc()
                        "Text marked or highlighted for reference."
                    }
                    div {
                        class: c_tag_demo_box()
                        "This is "
                        mark {
                            class: c_tag_demo_mark()
                            "highlighted"
                        }
                        " text."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<del> + <ins>"
                    }
                    p {
                        class: c_tag_desc()
                        "Removed and inserted text edits."
                    }
                    div {
                        class: c_tag_demo_box()
                        del {
                            "Deleted text"
                        }
                        " → "
                        ins {
                            "Inserted text"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<sub> / <sup>"
                    }
                    p {
                        class: c_tag_desc()
                        "Subscript and superscript text."
                    }
                    div {
                        class: c_tag_demo_box()
                        "H"
                        sub {
                            "2"
                        }
                        "O is water. E = mc"
                        sup {
                            "2"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<code>"
                    }
                    p {
                        class: c_tag_desc()
                        "A fragment of computer code."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Use "
                        code {
                            class: c_tag_demo_code()
                            "console.log()"
                        }
                        " for debugging."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<kbd>"
                    }
                    p {
                        class: c_tag_desc()
                        "User input via keyboard."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Press "
                        kbd {
                            class: c_tag_demo_kbd()
                            "Ctrl"
                        }
                        " + "
                        kbd {
                            class: c_tag_demo_kbd()
                            "S"
                        }
                        " to save."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<samp>"
                    }
                    p {
                        class: c_tag_desc()
                        "Sample output from a program."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Output: "
                        samp {
                            class: c_tag_demo_code()
                            "Hello, world!"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<var>"
                    }
                    p {
                        class: c_tag_desc()
                        "A variable in a mathematical or programming context."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Solve for "
                        var {
                            "x"
                        }
                        " in the equation."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<abbr>"
                    }
                    p {
                        class: c_tag_desc()
                        "An abbreviation or acronym."
                    }
                    div {
                        class: c_tag_demo_box()
                        abbr {
                            title: TAGS_ABBR_TITLE
                            "HTML"
                        }
                        " is the standard markup language."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<cite>"
                    }
                    p {
                        class: c_tag_desc()
                        "The title of a creative work."
                    }
                    div {
                        class: c_tag_demo_box()
                        "My favorite book is "
                        cite {
                            "The Rust Programming Language"
                        }
                        "."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<q>"
                    }
                    p {
                        class: c_tag_desc()
                        "An inline short quotation."
                    }
                    div {
                        class: c_tag_demo_box()
                        q {
                            "To be or not to be"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<dfn>"
                    }
                    p {
                        class: c_tag_desc()
                        "The defining instance of a term."
                    }
                    div {
                        class: c_tag_demo_box()
                        dfn {
                            "WASM"
                        }
                        " is a binary instruction format for a stack-based virtual machine."
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<time>"
                    }
                    p {
                        class: c_tag_desc()
                        "A specific period in time."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Published on "
                        time {
                            datetime: TAGS_TIME_DATETIME
                            "June 8, 2026"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<data>"
                    }
                    p {
                        class: c_tag_desc()
                        "Links content with a machine-readable translation."
                    }
                    div {
                        class: c_tag_demo_box()
                        data {
                            value: TAGS_DATA_VALUE
                            "Product ID: 398"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<ruby> / <rt> / <rp>"
                    }
                    p {
                        class: c_tag_desc()
                        "Ruby annotations for East Asian typography."
                    }
                    div {
                        class: c_tag_demo_box()
                        ruby {
                            "漢"
                            rp {
                                "("
                            }
                            rt {
                                "kan"
                            }
                            rp {
                                ")"
                            }
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<bdi> / <bdo>"
                    }
                    p {
                        class: c_tag_desc()
                        "Bi-directional text isolation and override."
                    }
                    div {
                        class: c_tag_demo_box()
                        bdi {
                            "إيان"
                        }
                        ": 90 points"
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<wbr>"
                    }
                    p {
                        class: c_tag_desc()
                        "A word break opportunity."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Superlong"
                        wbr {}
                        "wordbreak"
                        wbr {}
                        "opportunity"
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<br>"
                    }
                    p {
                        class: c_tag_desc()
                        "A line break."
                    }
                    div {
                        class: c_tag_demo_box()
                        "Line one"
                        br {}
                        "Line two"
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // List Content
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_LIST_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<ul>"
                    }
                    p {
                        class: c_tag_desc()
                        "An unordered list."
                    }
                    ul {
                        class: c_tag_demo_list()
                        li {
                            "Unordered item one"
                        }
                        li {
                            "Unordered item two"
                        }
                        li {
                            "Unordered item three"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<ol>"
                    }
                    p {
                        class: c_tag_desc()
                        "An ordered list."
                    }
                    ol {
                        class: c_tag_demo_list()
                        li {
                            "Ordered item one"
                        }
                        li {
                            "Ordered item two"
                        }
                        li {
                            "Ordered item three"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<dl> / <dt> / <dd>"
                    }
                    p {
                        class: c_tag_desc()
                        "A description / definition list."
                    }
                    dl {
                        class: c_tag_demo_dl()
                        dt {
                            class: c_tag_demo_dt()
                            "Term 1"
                        }
                        dd {
                            class: c_tag_demo_dd()
                            "Description for term 1"
                        }
                        dt {
                            class: c_tag_demo_dt()
                            "Term 2"
                        }
                        dd {
                            class: c_tag_demo_dd()
                            "Description for term 2"
                        }
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // Table Content
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_TABLE_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<table>"
                    }
                    p {
                        class: c_tag_desc()
                        "Data with more than one dimension, using caption, thead, tbody, tfoot, tr, th, and td."
                    }
                    table {
                        class: c_tag_demo_table()
                        caption {
                            class: c_tag_demo_caption()
                            "Framework Comparison"
                        }
                        thead {
                            tr {
                                th {
                                    class: c_tag_demo_th()
                                    "Name"
                                }
                                th {
                                    class: c_tag_demo_th()
                                    "Language"
                                }
                                th {
                                    class: c_tag_demo_th()
                                    "Type"
                                }
                            }
                        }
                        tbody {
                            tr {
                                td {
                                    class: c_tag_demo_td()
                                    "euv"
                                }
                                td {
                                    class: c_tag_demo_td()
                                    "Rust"
                                }
                                td {
                                    class: c_tag_demo_td()
                                    "WASM"
                                }
                            }
                            tr {
                                td {
                                    class: c_tag_demo_td()
                                    "React"
                                }
                                td {
                                    class: c_tag_demo_td()
                                    "JavaScript"
                                }
                                td {
                                    class: c_tag_demo_td()
                                    "DOM"
                                }
                            }
                        }
                        tfoot {
                            tr {
                                td {
                                    class: c_tag_demo_td()
                                    colspan: TAGS_TABLE_COLSPAN
                                    "2 frameworks compared"
                                }
                            }
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<colgroup> / <col>"
                    }
                    p {
                        class: c_tag_desc()
                        "Column groups and styling for tables."
                    }
                    table {
                        class: c_tag_demo_table()
                        colgroup {
                            col {
                                style: TAGS_COLGROUP_STYLE_A
                            }
                            col {
                                style: TAGS_COLGROUP_STYLE_B
                            }
                            col {
                                style: TAGS_COLGROUP_STYLE_C
                            }
                        }
                        tr {
                            th {
                                class: c_tag_demo_th()
                                "Column A"
                            }
                            th {
                                class: c_tag_demo_th()
                                "Column B"
                            }
                            th {
                                class: c_tag_demo_th()
                                "Column C"
                            }
                        }
                        tr {
                            td {
                                class: c_tag_demo_td()
                                "A1"
                            }
                            td {
                                class: c_tag_demo_td()
                                "B1"
                            }
                            td {
                                class: c_tag_demo_td()
                                "C1"
                            }
                        }
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // Embedded Content
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_EMBEDDED_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<img>"
                    }
                    p {
                        class: c_tag_desc()
                        "An image embed."
                    }
                    div {
                        class: c_tag_demo_box()
                        img {
                            class: c_tag_demo_img()
                            src: TAGS_IMAGE_SRC
                            alt: TAGS_IMG_ALT
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<picture> + <source>"
                    }
                    p {
                        class: c_tag_desc()
                        "A container for multiple image sources."
                    }
                    div {
                        class: c_tag_demo_box()
                        picture {
                            source {
                                srcset: TAGS_IMAGE_SRC
                                media: TAGS_PICTURE_MEDIA
                            }
                            img {
                                class: c_tag_demo_img()
                                src: TAGS_IMAGE_SRC
                                alt: TAGS_PICTURE_IMG_ALT
                            }
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<iframe>"
                    }
                    p {
                        class: c_tag_desc()
                        "A nested browsing context."
                    }
                    div {
                        class: c_tag_demo_box()
                        iframe {
                            class: c_tag_demo_iframe()
                            src: TAGS_BLANK_SRC
                            title: TAGS_IFRAME_TITLE
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<embed>"
                    }
                    p {
                        class: c_tag_desc()
                        "An external application or interactive content."
                    }
                    div {
                        class: c_tag_demo_box()
                        embed {
                            class: c_tag_demo_embed()
                            src: TAGS_BLANK_SRC
                            type: TAGS_EMBED_TYPE
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<object>"
                    }
                    p {
                        class: c_tag_desc()
                        "An external resource represented by images, nested browsing contexts, or plugins."
                    }
                    div {
                        class: c_tag_demo_box()
                        object {
                            class: c_tag_demo_embed()
                            data: TAGS_BLANK_SRC
                            type: TAGS_EMBED_TYPE
                            "Object fallback text"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<canvas>"
                    }
                    p {
                        class: c_tag_desc()
                        "A bitmap canvas for rendering graphics on the fly."
                    }
                    div {
                        class: c_tag_demo_box()
                        canvas {
                            class: c_tag_demo_canvas()
                            width: TAGS_CANVAS_WIDTH
                            height: TAGS_CANVAS_HEIGHT
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<svg>"
                    }
                    p {
                        class: c_tag_desc()
                        "A container for SVG graphics."
                    }
                    div {
                        class: c_tag_demo_box()
                        svg {
                            class: c_tag_demo_svg()
                            width: TAGS_SVG_WIDTH
                            height: TAGS_SVG_HEIGHT
                            viewBox: TAGS_SVG_VIEWBOX
                            xmlns: TAGS_SVG_XMLNS
                            circle {
                                cx: TAGS_SVG_CIRCLE_CX
                                cy: TAGS_SVG_CIRCLE_CY
                                r: TAGS_SVG_CIRCLE_R
                                fill: TAGS_SVG_CIRCLE_FILL
                                opacity: TAGS_SVG_CIRCLE_OPACITY
                            }
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<math>"
                    }
                    p {
                        class: c_tag_desc()
                        "MathML element for mathematical expressions."
                    }
                    div {
                        class: c_tag_demo_box()
                        math {
                            "x = \u{2212}b \u{00B1} \u{221A}(b\u{00B2} \u{2212} 4ac)"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<video>"
                    }
                    p {
                        class: c_tag_desc()
                        "Video playback with optional sources."
                    }
                    div {
                        class: c_tag_demo_box()
                        video {
                            class: c_tag_demo_video()
                            controls: TAGS_CONTROLS_ATTR
                            preload: TAGS_PRELOAD_METADATA
                            src: TAGS_VIDEO_SRC
                            "Your browser does not support video."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<audio>"
                    }
                    p {
                        class: c_tag_desc()
                        "Audio playback with optional sources."
                    }
                    div {
                        class: c_tag_demo_box()
                        audio {
                            class: c_tag_demo_audio()
                            controls: TAGS_CONTROLS_ATTR
                            src: TAGS_AUDIO_SRC
                            "Your browser does not support audio."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<map> + <area>"
                    }
                    p {
                        class: c_tag_desc()
                        "An image map with clickable areas."
                    }
                    div {
                        class: c_tag_demo_box()
                        map {
                            name: TAGS_MAP_NAME
                            area {
                                shape: TAGS_AREA_SHAPE
                                coords: TAGS_AREA_COORDS
                                href: TAGS_HREF_PLACEHOLDER
                                alt: TAGS_AREA_ALT
                            }
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<track>"
                    }
                    p {
                        class: c_tag_desc()
                        "Timed text track for media elements (used with video/audio)."
                    }
                    div {
                        class: c_tag_demo_box()
                        p {
                            class: c_tag_demo_text()
                            "(track is a child of video/audio, shown above in the video example context)"
                        }
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // Form Elements
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_FORM_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<form>"
                    }
                    p {
                        class: c_tag_desc()
                        "A section containing interactive controls for submitting information."
                    }
                }
                form {
                    class: c_tag_demo_form()
                    onsubmit: on_form_submit
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<input>"
                        }
                        p {
                            class: c_tag_desc()
                            "Typed data entry fields."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_TEXT_ID
                                "text:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_TEXT_ID
                                type: TAGS_INPUT_TYPE_TEXT
                                placeholder: TAGS_INPUT_TEXT_PLACEHOLDER
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_PASSWORD_ID
                                "password:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_PASSWORD_ID
                                type: TAGS_INPUT_TYPE_PASSWORD
                                placeholder: TAGS_INPUT_PASSWORD_PLACEHOLDER
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_EMAIL_ID
                                "email:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_EMAIL_ID
                                type: TAGS_INPUT_TYPE_EMAIL
                                placeholder: TAGS_INPUT_EMAIL_PLACEHOLDER
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_NUMBER_ID
                                "number:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_NUMBER_ID
                                type: TAGS_INPUT_TYPE_NUMBER
                                placeholder: TAGS_INPUT_NUMBER_PLACEHOLDER
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_DATE_ID
                                "date:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_DATE_ID
                                type: TAGS_INPUT_TYPE_DATE
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_TIME_ID
                                "time:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_TIME_ID
                                type: TAGS_INPUT_TYPE_TIME
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_DATETIME_LOCAL_ID
                                "datetime-local:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_DATETIME_LOCAL_ID
                                type: TAGS_INPUT_TYPE_DATETIME_LOCAL
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_MONTH_ID
                                "month:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_MONTH_ID
                                type: TAGS_INPUT_TYPE_MONTH
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_WEEK_ID
                                "week:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_WEEK_ID
                                type: TAGS_INPUT_TYPE_WEEK
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_TEL_ID
                                "tel:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_TEL_ID
                                type: TAGS_INPUT_TYPE_TEL
                                placeholder: TAGS_INPUT_TEL_PLACEHOLDER
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_URL_ID
                                "url:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_URL_ID
                                type: TAGS_INPUT_TYPE_URL
                                placeholder: TAGS_INPUT_URL_PLACEHOLDER
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_SEARCH_ID
                                "search:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_SEARCH_ID
                                type: TAGS_INPUT_TYPE_SEARCH
                                placeholder: TAGS_INPUT_SEARCH_PLACEHOLDER
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_RANGE_ID
                                "range:"
                            }
                            input {
                                class: c_tag_demo_input_range()
                                id: TAGS_INPUT_RANGE_ID
                                type: TAGS_INPUT_TYPE_RANGE
                                min: TAGS_RANGE_MIN
                                max: TAGS_RANGE_MAX
                                value: TAGS_RANGE_VALUE
                                style: format!("--value: {}%", TAGS_RANGE_VALUE)
                                oninput: range_on_input_update_style()
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_COLOR_ID
                                "color:"
                            }
                            input {
                                class: c_tag_demo_input_color()
                                id: TAGS_INPUT_COLOR_ID
                                type: TAGS_INPUT_TYPE_COLOR
                                value: TAGS_COLOR_INPUT_VALUE
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_CHECKBOX_ID
                                "checkbox:"
                            }
                            input {
                                id: TAGS_INPUT_CHECKBOX_ID
                                type: TAGS_INPUT_TYPE_CHECKBOX
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_RADIO_A_ID
                                "radio:"
                            }
                            input {
                                id: TAGS_INPUT_RADIO_A_ID
                                type: TAGS_INPUT_TYPE_RADIO
                                name: TAGS_RADIO_NAME
                                value: TAGS_RADIO_A_VALUE
                            }
                            " A "
                            input {
                                id: TAGS_INPUT_RADIO_B_ID
                                type: TAGS_INPUT_TYPE_RADIO
                                name: TAGS_RADIO_NAME
                                value: TAGS_RADIO_B_VALUE
                            }
                            " B"
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_INPUT_FILE_ID
                                "file:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_INPUT_FILE_ID
                                type: TAGS_INPUT_TYPE_FILE
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            input {
                                type: TAGS_INPUT_TYPE_HIDDEN
                                value: TAGS_HIDDEN_INPUT_VALUE
                            }
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<textarea>"
                        }
                        p {
                            class: c_tag_desc()
                            "A multi-line plain text editing control."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_TEXTAREA_ID
                                "textarea:"
                            }
                            textarea {
                                class: c_tag_demo_textarea()
                                id: TAGS_TEXTAREA_ID
                                rows: TAGS_TEXTAREA_ROWS
                                placeholder: TAGS_TEXTAREA_PLACEHOLDER
                            }
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<select> / <option> / <optgroup>"
                        }
                        p {
                            class: c_tag_desc()
                            "A dropdown selection control with optional groups."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_SELECT_ID
                                "select:"
                            }
                            select {
                                class: c_tag_demo_select()
                                id: TAGS_SELECT_ID
                                optgroup {
                                    label: TAGS_OPTGROUP_FRONTEND
                                    option {
                                        value: TAGS_SELECT_VALUE_RUST
                                        "Rust"
                                    }
                                    option {
                                        value: TAGS_SELECT_VALUE_JAVASCRIPT
                                        "JavaScript"
                                    }
                                }
                                optgroup {
                                    label: TAGS_OPTGROUP_BACKEND
                                    option {
                                        value: TAGS_SELECT_VALUE_GO
                                        "Go"
                                    }
                                    option {
                                        value: TAGS_SELECT_VALUE_PYTHON
                                        "Python"
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<datalist>"
                        }
                        p {
                            class: c_tag_desc()
                            "A set of pre-defined options for an input field."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            input {
                                class: c_tag_demo_input()
                                list: TAGS_DATALIST_ID
                                placeholder: TAGS_INPUT_DATALIST_PLACEHOLDER
                            }
                            datalist {
                                id: TAGS_DATALIST_ID
                                option {
                                    value: TAGS_DATALIST_VALUE_CHROME
                                }
                                option {
                                    value: TAGS_DATALIST_VALUE_FIREFOX
                                }
                                option {
                                    value: TAGS_DATALIST_VALUE_SAFARI
                                }
                            }
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<output>"
                        }
                        p {
                            class: c_tag_desc()
                            "A container for the result of a calculation."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            output {
                                class: c_tag_demo_code()
                                name: TAGS_OUTPUT_NAME
                                "42"
                            }
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<progress>"
                        }
                        p {
                            class: c_tag_desc()
                            "A progress bar indicating completion."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            progress {
                                class: c_tag_demo_progress()
                                value: TAGS_PROGRESS_VALUE
                                max: TAGS_PROGRESS_MAX
                            }
                            " 70%"
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<meter>"
                        }
                        p {
                            class: c_tag_desc()
                            "A scalar measurement within a known range."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            meter {
                                class: c_tag_demo_meter()
                                value: TAGS_METER_VALUE
                                min: TAGS_METER_MIN
                                max: TAGS_METER_MAX
                                low: TAGS_METER_LOW
                                high: TAGS_METER_HIGH
                                optimum: TAGS_METER_OPTIMUM
                            }
                            " Disk usage: 70%"
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<fieldset> + <legend>"
                        }
                        p {
                            class: c_tag_desc()
                            "A group of related form controls with a caption."
                        }
                        fieldset {
                            class: c_tag_demo_fieldset()
                            legend {
                                class: c_tag_demo_legend()
                                "Personal Info"
                            }
                            div {
                                class: c_euv_input_wrapper()
                                label {
                                    class: c_form_label()
                                    for: TAGS_FIELDSET_NAME_ID
                                    "Name:"
                                }
                                input {
                                    class: c_tag_demo_input()
                                    id: TAGS_FIELDSET_NAME_ID
                                    type: TAGS_INPUT_TYPE_TEXT
                                    placeholder: TAGS_FIELDSET_NAME_PLACEHOLDER
                                }
                            }
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<button>"
                        }
                        p {
                            class: c_tag_desc()
                            "A clickable button."
                        }
                        div {
                            class: c_tag_button_row()
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: TAGS_BUTTON_CLICK
                                TAGS_BUTTON_CLICK
                            }
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: TAGS_BUTTON_SUBMIT
                                TAGS_BUTTON_SUBMIT
                            }
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: TAGS_BUTTON_RESET
                                TAGS_BUTTON_RESET
                            }
                        }
                    }
                    div {
                        class: c_tag_section()
                        h4 {
                            class: c_tag_group_title()
                            "<label>"
                        }
                        p {
                            class: c_tag_desc()
                            "A caption for an item in a user interface."
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                class: c_form_label()
                                for: TAGS_LABEL_DEMO_ID
                                "Click to focus:"
                            }
                            input {
                                class: c_tag_demo_input()
                                id: TAGS_LABEL_DEMO_ID
                                type: TAGS_INPUT_TYPE_TEXT
                                placeholder: TAGS_LABEL_DEMO_PLACEHOLDER
                            }
                        }
                    }
                }
            }
            // ═══════════════════════════════════════════════════════════════
            // Interactive / Scripting
            // ═══════════════════════════════════════════════════════════════
            euv_card {
                title: TAGS_INTERACTIVE_TITLE
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<menu>"
                    }
                    p {
                        class: c_tag_desc()
                        "A list of commands (toolbar or context menu)."
                    }
                    menu {
                        class: c_tag_demo_menu()
                        li {
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: TAGS_MENU_COMMAND_FIRST
                                TAGS_MENU_COMMAND_FIRST
                            }
                        }
                        li {
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: TAGS_MENU_COMMAND_SECOND
                                TAGS_MENU_COMMAND_SECOND
                            }
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<script> / <noscript>"
                    }
                    p {
                        class: c_tag_desc()
                        "Executable scripts and fallback for disabled scripting."
                    }
                    div {
                        class: c_tag_demo_box()
                        noscript {
                            "JavaScript is disabled in your browser."
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<template> / <slot>"
                    }
                    p {
                        class: c_tag_desc()
                        "Content templates and Web Component slots."
                    }
                    div {
                        class: c_tag_demo_box()
                        template {
                            div {
                                "Template content (not rendered directly)"
                            }
                        }
                        p {
                            class: c_tag_demo_text()
                            "(template and slot are used by Web Components, not visible in normal DOM)"
                        }
                    }
                }
                div {
                    class: c_tag_section()
                    h4 {
                        class: c_tag_group_title()
                        "<portal>"
                    }
                    p {
                        class: c_tag_desc()
                        "Enables navigation to another page (experimental)."
                    }
                    div {
                        class: c_tag_demo_box()
                        p {
                            class: c_tag_demo_text()
                            "(portal is an experimental element, not widely supported)"
                        }
                    }
                }
            }
        }
    }
}
