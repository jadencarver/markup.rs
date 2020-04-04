use markup;

markup::define!(
    TestNamespace {
        xml:testing {
            "1 2 3"
        }
    }
);

#[test]
fn it_supports_namespaces() {
    let result = TestNamespace {};
    let result_s = result.to_string();
    assert_eq!(result_s, "<xml:testing>1 2 3</xml:testing>");
}
