pub use fetch::fetch_app;
pub use rules::num_version;

mod fetch;
mod rule_index;
mod rules;

type FnSignature = fn(&str) -> Option<String>;
