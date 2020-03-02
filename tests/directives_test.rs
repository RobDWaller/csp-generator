#[cfg(test)]
mod directives_test {
    extern crate csp_generator;
    use csp_generator::directives;
    use csp_generator::config;

    #[test]
    fn test_build_directives() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src"]},
                    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        let csp: String = directives::build(config::get_directives(), json);

        assert_eq!(csp, String::from("script-src: test.com; connect-src: example.com test.com;"));
    }
}