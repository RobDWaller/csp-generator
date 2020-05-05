use crate::domains::Collection;
use serde_json::error;

// Parse the supplied CSP JSON config to a collection of domains and directives.
pub fn json(json: &str) -> Result<Collection, error::Error> {
    let result = serde_json::from_str(json);

    match result {
        Ok(result) => {
            let parsed: Collection = result;
            Ok(parsed)
        }
        Err(e) => Err(e),
    }
}

// -----
// Tests
// -----
#[cfg(test)]
mod parse_test {
    #[test]
    fn test_parse_json() {
        let json = r#"
            [
                {"domain": "example.com", "directives": ["connect-src", "script-src"]},
                {"domain": "test.com", "directives": ["script-src", "img-src", "style-src"]}
            ]
        "#;

        let domains = super::json(json).unwrap();

        assert_eq!(domains[0].domain, "example.com");
        assert_eq!(domains[1].domain, "test.com");
        assert_eq!(domains[1].directives[1], "img-src");
    }

    #[test]
    fn test_parse_json_empty() {
        let json = r#""#;

        let domains = super::json(json);

        assert!(domains.is_err());
    }

    #[test]
    fn test_parse_json_bad() {
        let json = r#"
            [
                {"domain": "example.com", "diroctive": ["connect-src", "script-src"]}
            ]
        "#;

        let domains = super::json(json);

        assert!(domains.is_err());
    }
}
