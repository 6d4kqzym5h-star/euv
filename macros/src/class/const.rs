/// The keyword name for the `nth_child` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_NTH_CHILD: &str = "nth_child";

/// The keyword name for the `nth_last_child` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_NTH_LAST_CHILD: &str = "nth_last_child";

/// The keyword name for the `media` query block in class macro syntax.
pub(crate) const KEYWORD_MEDIA: &str = "media";

/// The keyword name for the `hover` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_HOVER: &str = "hover";

/// The keyword name for the `focus` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_FOCUS: &str = "focus";

/// The keyword name for the `focus_within` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_FOCUS_WITHIN: &str = "focus_within";

/// The keyword name for the `focus_visible` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_FOCUS_VISIBLE: &str = "focus_visible";

/// The keyword name for the `active` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_ACTIVE: &str = "active";

/// The keyword name for the `visited` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_VISITED: &str = "visited";

/// The keyword name for the `disabled` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_DISABLED: &str = "disabled";

/// The keyword name for the `enabled` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_ENABLED: &str = "enabled";

/// The keyword name for the `checked` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_CHECKED: &str = "checked";

/// The keyword name for the `readonly` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_READONLY: &str = "readonly";

/// The keyword name for the `readwrite` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_READWRITE: &str = "readwrite";

/// The keyword name for the `required` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_REQUIRED: &str = "required";

/// The keyword name for the `optional` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_OPTIONAL: &str = "optional";

/// The keyword name for the `valid` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_VALID: &str = "valid";

/// The keyword name for the `invalid` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_INVALID: &str = "invalid";

/// The keyword name for the `in_range` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_IN_RANGE: &str = "in_range";

/// The keyword name for the `out_of_range` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_OUT_OF_RANGE: &str = "out_of_range";

/// The keyword name for the `placeholder_shown` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_PLACEHOLDER_SHOWN: &str = "placeholder_shown";

/// The keyword name for the `first_child` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_FIRST_CHILD: &str = "first_child";

/// The keyword name for the `last_child` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_LAST_CHILD: &str = "last_child";

/// The keyword name for the `only_child` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_ONLY_CHILD: &str = "only_child";

/// The keyword name for the `first_of_type` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_FIRST_OF_TYPE: &str = "first_of_type";

/// The keyword name for the `last_of_type` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_LAST_OF_TYPE: &str = "last_of_type";

/// The keyword name for the `only_of_type` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_ONLY_OF_TYPE: &str = "only_of_type";

/// The keyword name for the `root` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_ROOT: &str = "root";

/// The keyword name for the `empty` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_EMPTY: &str = "empty";

/// The keyword name for the `target` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_TARGET: &str = "target";

/// The keyword name for the `link` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_LINK: &str = "link";

/// The keyword name for the `any_link` pseudo-class in class macro syntax.
pub(crate) const KEYWORD_ANY_LINK: &str = "any_link";

/// The keyword name for the `before` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_BEFORE: &str = "before";

/// The keyword name for the `after` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_AFTER: &str = "after";

/// The keyword name for the `first_line` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_FIRST_LINE: &str = "first_line";

/// The keyword name for the `first_letter` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_FIRST_LETTER: &str = "first_letter";

/// The keyword name for the `selection` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_SELECTION: &str = "selection";

/// The keyword name for the `placeholder` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_PLACEHOLDER: &str = "placeholder";

/// The keyword name for the `backdrop` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_BACKDROP: &str = "backdrop";

/// The keyword name for the `marker` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_MARKER: &str = "marker";

/// The keyword name for the `spelling_error` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_SPELLING_ERROR: &str = "spelling_error";

/// The keyword name for the `grammar_error` pseudo-element in class macro syntax.
pub(crate) const KEYWORD_GRAMMAR_ERROR: &str = "grammar_error";

/// The CSS `@media` rule prefix used in serialized media query strings.
pub(crate) const CSS_MEDIA_PREFIX: &str = "@media ";

/// The CSS pseudo-rule serialization separator between selector and style block.
pub(crate) const CSS_RULE_OPEN: &str = " { ";

/// The CSS property separator between property name and value.
pub(crate) const CSS_PROP_SEPARATOR: &str = ": ";

/// The CSS declaration terminator after each property value.
pub(crate) const CSS_DECL_TERMINATOR: &str = "; ";

/// The space string used for concatenating CSS parts.
pub(crate) const STR_SPACE: &str = " ";

/// The keyword name for the `var` function in class macro syntax.
pub(crate) const VAR: &str = "var";

/// The CSS `var()` function prefix for custom property references.
pub(crate) const CSS_VAR_PREFIX: &str = "var(--";

/// The CSS `var()` function suffix for custom property references.
pub(crate) const CSS_VAR_SUFFIX: &str = ")";

/// The `format` macro name used to detect format! invocations in class property values.
pub(crate) const FORMAT_MACRO: &str = "format";

/// The CSS pseudo selector for `:hover`.
pub(crate) const PSEUDO_HOVER: &str = ":hover";

/// The CSS pseudo selector for `:focus`.
pub(crate) const PSEUDO_FOCUS: &str = ":focus";

/// The CSS pseudo selector for `:focus-within`.
pub(crate) const PSEUDO_FOCUS_WITHIN: &str = ":focus-within";

/// The CSS pseudo selector for `:focus-visible`.
pub(crate) const PSEUDO_FOCUS_VISIBLE: &str = ":focus-visible";

/// The CSS pseudo selector for `:active`.
pub(crate) const PSEUDO_ACTIVE: &str = ":active";

/// The CSS pseudo selector for `:visited`.
pub(crate) const PSEUDO_VISITED: &str = ":visited";

/// The CSS pseudo selector for `:disabled`.
pub(crate) const PSEUDO_DISABLED: &str = ":disabled";

/// The CSS pseudo selector for `:enabled`.
pub(crate) const PSEUDO_ENABLED: &str = ":enabled";

/// The CSS pseudo selector for `:checked`.
pub(crate) const PSEUDO_CHECKED: &str = ":checked";

/// The CSS pseudo selector for `:read-only`.
pub(crate) const PSEUDO_READONLY: &str = ":read-only";

/// The CSS pseudo selector for `:read-write`.
pub(crate) const PSEUDO_READWRITE: &str = ":read-write";

/// The CSS pseudo selector for `:required`.
pub(crate) const PSEUDO_REQUIRED: &str = ":required";

/// The CSS pseudo selector for `:optional`.
pub(crate) const PSEUDO_OPTIONAL: &str = ":optional";

/// The CSS pseudo selector for `:valid`.
pub(crate) const PSEUDO_VALID: &str = ":valid";

/// The CSS pseudo selector for `:invalid`.
pub(crate) const PSEUDO_INVALID: &str = ":invalid";

/// The CSS pseudo selector for `:in-range`.
pub(crate) const PSEUDO_IN_RANGE: &str = ":in-range";

/// The CSS pseudo selector for `:out-of-range`.
pub(crate) const PSEUDO_OUT_OF_RANGE: &str = ":out-of-range";

/// The CSS pseudo selector for `:placeholder-shown`.
pub(crate) const PSEUDO_PLACEHOLDER_SHOWN: &str = ":placeholder-shown";

/// The CSS pseudo selector for `:first-child`.
pub(crate) const PSEUDO_FIRST_CHILD: &str = ":first-child";

/// The CSS pseudo selector for `:last-child`.
pub(crate) const PSEUDO_LAST_CHILD: &str = ":last-child";

/// The CSS pseudo selector for `:only-child`.
pub(crate) const PSEUDO_ONLY_CHILD: &str = ":only-child";

/// The CSS pseudo selector for `:first-of-type`.
pub(crate) const PSEUDO_FIRST_OF_TYPE: &str = ":first-of-type";

/// The CSS pseudo selector for `:last-of-type`.
pub(crate) const PSEUDO_LAST_OF_TYPE: &str = ":last-of-type";

/// The CSS pseudo selector for `:only-of-type`.
pub(crate) const PSEUDO_ONLY_OF_TYPE: &str = ":only-of-type";

/// The CSS pseudo selector for `:root`.
pub(crate) const PSEUDO_ROOT: &str = ":root";

/// The CSS pseudo selector for `:empty`.
pub(crate) const PSEUDO_EMPTY: &str = ":empty";

/// The CSS pseudo selector for `:target`.
pub(crate) const PSEUDO_TARGET: &str = ":target";

/// The CSS pseudo selector for `:link`.
pub(crate) const PSEUDO_LINK: &str = ":link";

/// The CSS pseudo selector for `:any-link`.
pub(crate) const PSEUDO_ANY_LINK: &str = ":any-link";

/// The CSS pseudo element selector for `::before`.
pub(crate) const PSEUDO_BEFORE: &str = "::before";

/// The CSS pseudo element selector for `::after`.
pub(crate) const PSEUDO_AFTER: &str = "::after";

/// The CSS pseudo element selector for `::first-line`.
pub(crate) const PSEUDO_FIRST_LINE: &str = "::first-line";

/// The CSS pseudo element selector for `::first-letter`.
pub(crate) const PSEUDO_FIRST_LETTER: &str = "::first-letter";

/// The CSS pseudo element selector for `::selection`.
pub(crate) const PSEUDO_SELECTION: &str = "::selection";

/// The CSS pseudo element selector for `::placeholder`.
pub(crate) const PSEUDO_PLACEHOLDER: &str = "::placeholder";

/// The CSS pseudo element selector for `::backdrop`.
pub(crate) const PSEUDO_BACKDROP: &str = "::backdrop";

/// The CSS pseudo element selector for `::marker`.
pub(crate) const PSEUDO_MARKER: &str = "::marker";

/// The CSS pseudo element selector for `::spelling-error`.
pub(crate) const PSEUDO_SPELLING_ERROR: &str = "::spelling-error";

/// The CSS pseudo element selector for `::grammar-error`.
pub(crate) const PSEUDO_GRAMMAR_ERROR: &str = "::grammar-error";

/// The hyphen string used for replacing underscores in kebab-case conversion.
pub(crate) const STR_HYPHEN: &str = "-";

/// The closing brace character used to terminate CSS rule blocks.
pub(crate) const CHAR_CSS_RULE_CLOSE: char = '}';
