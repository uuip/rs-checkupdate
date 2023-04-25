use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::rules;
use crate::FnSignature;

pub static RULES: Lazy<HashMap<&'static str, FnSignature>> = Lazy::new(|| {
    let mapper: [(&str, FnSignature); 20] = [
        ("AIDA64", rules::parse_aida64),
        ("Beyond Compare", rules::parse_beyond_compare),
        ("Contexts [Mac]", rules::parse_contexts_mac),
        ("DevManView", rules::parse_dev_man_view),
        ("EmEditor", rules::parse_emeditor),
        ("Everything", rules::parse_everything),
        ("FS Capture", rules::parse_faststone),
        ("FS Viewer", rules::parse_faststone),
        ("Firefox", rules::parse_firefox),
        ("Git", rules::parse_git),
        ("Navicat", rules::parse_navicat),
        ("Navicat[Mac]", rules::parse_navicat_mac),
        ("PDF-XChange", rules::parse_pdf_xchange),
        ("Python", rules::parse_python),
        ("Registry Workshop", rules::parse_registry_workshop),
        ("SecureCRT", rules::parse_secure_crt),
        ("VMware", rules::parse_vmware),
        ("WGestures 2", rules::parse_wgestures2),
        ("WGestures 2 [Mac]", rules::parse_wgestures2_mac),
        ("WinRAR", rules::parse_winrar),
    ];
    HashMap::from(mapper)
});
