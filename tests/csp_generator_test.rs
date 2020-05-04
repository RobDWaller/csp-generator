#[cfg(test)]
mod csp_generator_test {
    extern crate csp_generator;
    use csp_generator::directives;

    #[test]
    fn test_enforce() {
        let json = r#"
            [
                {"domain": "example.com", "directives": ["connect-src"]},
                {"domain": "test.com", "directives": ["connect-src", "script-src"]}
            ]
        "#;

        let csp: csp_generator::Csp = csp_generator::enforce(directives::directives(), json);

        assert_eq!(csp.header, String::from("Content-Security-Policy"));
        assert_eq!(
            csp.csp,
            String::from("script-src test.com; connect-src example.com test.com;")
        );
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: EOF while parsing a value at line 1 column 0")]
    fn test_enforce_empty_fail() {
        let json = "";

        csp_generator::enforce(directives::directives(), json);
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: missing field")]
    fn test_enforce_format_fail() {
        let json = r#"
            [
                {"domain": "example.com", "diroctive": ["connect-src"]},
                {"domain": "test.com", "directive": ["connect-src", "script-src"]}
            ]
        "#;

        csp_generator::enforce(directives::directives(), json);
    }

    #[test]
    fn test_report_only() {
        let json = r#"
            [
                {"domain": "example.com", "directives": ["connect-src"]},
                {"domain": "test.com", "directives": ["connect-src", "script-src"]}
            ]
        "#;

        let csp: csp_generator::Csp = csp_generator::report_only(directives::directives(), json);

        assert_eq!(
            csp.header,
            String::from("Content-Security-Policy-Report-Only")
        );
        assert_eq!(
            csp.csp,
            String::from("script-src test.com; connect-src example.com test.com;")
        );
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: EOF while parsing a value at line 1 column 0")]
    fn test_report_only_empty_fail() {
        let json = "";

        csp_generator::report_only(directives::directives(), json);
    }

    #[test]
    #[should_panic(expected = "Could not parse JSON: missing field")]
    fn test_report_only_format_fail() {
        let json = r#"
            [
                {"domain": "example.com", "directives": ["connect-src"]},
                {"directives": ["connect-src", "script-src"]}
            ]
        "#;

        csp_generator::report_only(directives::directives(), json);
    }

    #[test]
    fn test_csp_only() {
        let json = r#"
            [
                {"domain": "example.com", "directives": ["connect-src"]},
                {"domain": "test.com", "directives": ["connect-src", "script-src"]}
            ]
        "#;

        let csp: String = csp_generator::csp_only(directives::directives(), json);

        assert_eq!(
            csp,
            String::from("script-src test.com; connect-src example.com test.com;")
        );
    }
}
