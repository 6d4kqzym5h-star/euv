/// An entry for `I18n::add_messages` — a `(key, value)`
/// pair that the caller usually writes as a single
/// literal array. The tuple form keeps the public API
/// allocation-free for the common "compile-time literal
/// list of pairs" case.
pub type MessageEntry = (&'static str, &'static str);
