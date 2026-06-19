use fancy_regex::Regex;

pub fn split_address(address: &str) -> (&str, &str) {
    address
        .rsplit_once('!')
        .map(|(sheet_name, range)| (sheet_name.trim_matches(&['\'', '"'][..]), range))
        .unwrap_or(("", address))
}

pub fn join_address(sheet_name: &str, address: &str) -> String {
    if sheet_name == "" {
        return address.to_string();
    }
    if sheet_name_needs_quoting(sheet_name) {
        let escaped = sheet_name.replace('\'', "''");
        return format!("'{}'!{}", escaped, address);
    }
    format!("{}!{}", sheet_name, address)
}

// A sheet name can appear unquoted in a formula only when it is a bare
// identifier: it starts with a letter or underscore and otherwise contains
// only letters, digits, underscores or periods. Anything else (spaces,
// punctuation such as `(`/`)`, a leading digit, ...) must be wrapped in single
// quotes when re-serialized.
fn sheet_name_needs_quoting(sheet_name: &str) -> bool {
    let mut chars = sheet_name.chars();
    match chars.next() {
        None => false,
        Some(first) => {
            if !(first.is_ascii_alphabetic() || first == '_') {
                return true;
            }
            sheet_name
                .chars()
                .any(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'))
        }
    }
}

#[test]
fn split_address_test() {
    assert_eq!(split_address("A1"), ("", "A1"));
    assert_eq!(split_address("A1:B2"), ("", "A1:B2"));
    assert_eq!(split_address("sheet1!A1:B2"), ("sheet1", "A1:B2"));
    assert_eq!(split_address("'she!et1'!A1:B2"), ("she!et1", "A1:B2"));
    assert_eq!(split_address(r#"'she"et1'!A1:B2"#), (r#"she"et1"#, "A1:B2"));
}

pub fn is_address<S: AsRef<str>>(input: S) -> bool {
    let re =
        Regex::new(r"^([^\:\\\?\[\]\/\*]+\!)?(\$?[A-Z]{1,3}\$?[0-9]+)(\:\$?[A-Z]{1,3}\$?[0-9]+)?$")
            .unwrap();
    re.is_match(input.as_ref()).unwrap()
}

#[test]
fn is_address_test() {
    assert!(is_address("A1"));
    assert!(is_address("$A1"));
    assert!(is_address("A$1"));
    assert!(is_address("$A$1"));

    assert!(is_address("A1:B2"));
    assert!(is_address("$A1:B2"));
    assert!(is_address("$A$1:B2"));
    assert!(is_address("$A$1:$B2"));
    assert!(is_address("$A$1:$B$2"));

    assert!(is_address("Sheet1!A1"));
    assert!(is_address("Sheet1!$A1"));
    assert!(is_address("Sheet1!A$1"));
    assert!(is_address("Sheet1!A$1"));

    assert!(is_address("Sheet1!A1:B2"));
    assert!(is_address("Sheet1!$A1:B2"));
    assert!(is_address("Sheet1!$A$1:B2"));
    assert!(is_address("Sheet1!$A$1:$B2"));
    assert!(is_address("Sheet1!$A$1:$B$2"));
    assert!(is_address("New Sheet!$H$7:$H$10"));

    assert!(!is_address("(Sheet1!A1:B2)"));
    assert!(!is_address("Sheet1!A1:"));
    assert!(!is_address("Sheet1!A1:B"));
    assert!(!is_address("Sheet1!A:B2"));
    assert!(!is_address("Sheet1"));
}
