

#[test]
fn prometheusTag() {
    use super::PrometheusTags;
    use std::collections::HashMap;
    let mut tags = PrometheusTags(HashMap::new());
    tags.0.insert("hallo".to_string(), "welt".to_string());
    tags.0.insert("hello".to_string(), "world".to_string());
    assert_eq!(tags.to_string(), "hello=\"world\",hallo=\"welt\"".to_string());
}