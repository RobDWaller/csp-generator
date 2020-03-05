#[cfg(test)]
mod csp_generator_test {
    extern crate csp_generator;
    use csp_generator::config;

    #[test]
    fn test_enforce() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src"]},
                    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        let csp: String = csp_generator::enforce(config::get_directives(), json);

        assert_eq!(
            csp,
            String::from(
                "content-security-policy script-src: test.com; connect-src: example.com test.com;"
            )
        );
    }

    #[test]
    fn test_report_only() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src"]},
                    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        let csp: String = csp_generator::report_only(config::get_directives(), json);

        assert_eq!(
            csp,
            String::from(
                "content-security-policy-report-only script-src: test.com; connect-src: example.com test.com;"
            )
        );
    }
}