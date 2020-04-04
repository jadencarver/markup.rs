use markup;

markup::define!(
    TestNamespace1 {
        xml:testing {
            "1 2 3"
        }
    }
    TestNamespace2 {
        xsl:if[test="123"] {
            "Do Rah Me"
        }
    }
    TestNAmespace3 {
        xsl:"apply-templates"[select="/test"] {}
    }
);

#[test]
fn it_supports_namespaces() {
    let result = TestNamespace1 {};
    let result_s = result.to_string();
    assert_eq!(result_s, "<xml:testing>1 2 3</xml:testing>");
}

#[test]
fn it_supports_namespaces_with_attributes {
    let result = TestNamespace2 {};
    let result_s = result.to_string();
    assert_eq!(result_s, "<xsl:if test=\"123\">Do Rah Me</xsl:if>");
}

#[test]
fn it_supports_quoted_namespaces {
    let result = TestNamespace3 {};
    let result_s = result.to_string();
    assert_eq!(result_s, "<xsl:apply-templates select=\"/test\"></xsl:apply-templates>");
}
