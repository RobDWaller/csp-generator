use crate::domains;
use serde_json::error;

pub fn json(json: &str) -> Result<domains::Collection, error::Error> {
    let result = serde_json::from_str(json);

    match result {
        Ok(result) => {
            let parsed: domains::Collection = result;
            Ok(parsed)
        }
        Err(e) => Err(e),
    }
}

// -----
// Tests
// -----
#[cfg(test)]
mod parse_json_test {
    #[test]
    fn test_parse_json() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src", "script-src"]},
                    {"domain": "test.com", "directive": ["script-src", "img-src", "style-src"]}
                ]
            }
        "#;

        let domains = super::json(json).unwrap();

        assert_eq!(domains.domains[0].domain, "example.com");
        assert_eq!(domains.domains[1].domain, "test.com");
        assert_eq!(domains.domains[1].directive[1], "img-src");
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
            {
                "domains": [
                    {"domain": "example.com", "diroctive": ["connect-src", "script-src"]}
                ]
            }
        "#;

        let domains = super::json(json);

        assert!(domains.is_err());
    }
}
