fn main() {
    // ANCHOR: main_body
    let mut s = "foo".to_string();  // mut String
    dp!(s);

    let ss = &s;                    // &String
    dp!(ss);

    let ssl = &s[..];               // &str (= スライス)
    dp!(ssl);
    // ANCHOR_END: main_body
}
