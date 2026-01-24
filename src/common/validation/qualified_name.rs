use regex::Regex;
use std::sync::OnceLock;

use super::{BadValue, ErrorList, Path, invalid, is_dns1123_subdomain};

const QUALIFIED_NAME_FMT: &str = "([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9]";
const QUALIFIED_NAME_ERR_MSG: &str = "must consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character";
const QUALIFIED_NAME_MAX_LENGTH: usize = 63;

static QUALIFIED_NAME_RE: OnceLock<Regex> = OnceLock::new();

pub fn validate_qualified_name(value: &str, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_qualified_name(value) {
        all_errs.push(invalid(fld_path, BadValue::String(value.to_string()), &msg));
    }
    all_errs
}

pub fn is_qualified_name(value: &str) -> Vec<String> {
    let mut errs = Vec::new();
    let parts: Vec<&str> = value.split('/').collect();
    let name = match parts.len() {
        1 => parts[0],
        2 => {
            let prefix = parts[0];
            let name = parts[1];
            if prefix.is_empty() {
                errs.push(format!("prefix part {}", empty_error()));
            } else {
                let msgs = is_dns1123_subdomain(prefix);
                if !msgs.is_empty() {
                    errs.extend(prefix_each(msgs, "prefix part "));
                }
            }
            name
        }
        _ => {
            errs.push(format!(
                "a qualified name {} with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')",
                regex_error(
                    QUALIFIED_NAME_ERR_MSG,
                    QUALIFIED_NAME_FMT,
                    &["MyName", "my.name", "123-abc"],
                )
            ));
            return errs;
        }
    };

    if name.is_empty() {
        errs.push(format!("name part {}", empty_error()));
    } else if name.len() > QUALIFIED_NAME_MAX_LENGTH {
        errs.push(format!(
            "name part {}",
            max_len_error(QUALIFIED_NAME_MAX_LENGTH)
        ));
    }

    let re =
        QUALIFIED_NAME_RE.get_or_init(|| Regex::new(&format!("^{}$", QUALIFIED_NAME_FMT)).unwrap());
    if !re.is_match(name) {
        errs.push(format!(
            "name part {}",
            regex_error(
                QUALIFIED_NAME_ERR_MSG,
                QUALIFIED_NAME_FMT,
                &["MyName", "my.name", "123-abc"],
            )
        ));
    }

    errs
}

fn max_len_error(length: usize) -> String {
    format!("must be no more than {} characters", length)
}

fn regex_error(msg: &str, fmt: &str, examples: &[&str]) -> String {
    if examples.is_empty() {
        return format!("{} (regex used for validation is '{}')", msg, fmt);
    }

    let mut out = format!("{} (e.g. ", msg);
    for (i, example) in examples.iter().enumerate() {
        if i > 0 {
            out.push_str(" or ");
        }
        out.push('\'');
        out.push_str(example);
        out.push_str("', ");
    }
    out.push_str(&format!("regex used for validation is '{}')", fmt));
    out
}

fn empty_error() -> &'static str {
    "must be non-empty"
}

fn prefix_each(mut msgs: Vec<String>, prefix: &str) -> Vec<String> {
    for msg in &mut msgs {
        *msg = format!("{}{}", prefix, msg);
    }
    msgs
}
