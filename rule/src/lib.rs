pub use fetch::fetch_app;

mod fetch;
mod rule_index;
mod rules;

type FnSignature = fn(&str) -> Option<String>;
pub const UA: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:112.0) Gecko/20100101 Firefox/112.0";
