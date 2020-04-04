use markup;

markup::define!(
    TestNamespace {
        xml:testing {
            "1 2 3"
        }
        xsl:if[test="123"] {
            "Do Rah Me"
        }
    }
);

#[test]
fn it_supports_namespaces() {
    let result = TestNamespace {};
    let result_s = result.to_string();
    assert_eq!(result_s, "<xml:testing>1 2 3</xml:testing><xsl:if test=\"123\">Do Rah Me</xsl:if>");
}
