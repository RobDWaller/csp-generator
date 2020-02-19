use crate::csp_item;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct CspJson {
    pub domains: Vec<csp_item::CspItem>
}

pub fn json_to_csp(json: &str) -> Option<CspJson> {    
    let result = serde_json::from_str(json);

    if !result.is_err() {
        let parsed: CspJson = result.unwrap();

        return Some(parsed);
    }

    return None;
}

#[cfg(test)]
mod csp_json_test {
    #[test]
    fn test_json_to_csp() {
        let json = r#"
            {
                "domains": [
                    {"domain": "example.com", "directive": ["connect-src", "script-src"]},
                    {"domain": "test.com", "directive": ["script-src", "img-src", "style-src"]}
                ]
            }
        "#;

        let result = super::json_to_csp(json);

        let domains = result.unwrap(); 

        assert_eq!(domains.domains[0].domain, "example.com");
        assert_eq!(domains.domains[1].domain, "test.com");
        assert_eq!(domains.domains[1].directive[1], "img-src");
    }

    #[test]
    fn test_json_to_csp_empty() {
        let json = r#""#;

        let domains = super::json_to_csp(json);

        assert!(domains.is_none());
    }
}