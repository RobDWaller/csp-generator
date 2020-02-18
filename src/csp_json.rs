use crate::csp_item;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct CspJson {
    domains: Vec<csp_item::CspItem>
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
                    {"domain": "example.com", "directive": {"connect_src": true, "script_src": false, "img_src": false, "style_src": true, "object_src": true, "frame_src": false, "media_src": true, "script_src_elem": false, "worker_src": true}},
                    {"domain": "test.com", "directive": {"connect_src": true, "script_src": true, "img_src": false, "style_src": false, "object_src": true, "frame_src": true, "media_src": true, "script_src_elem": true, "worker_src": true}}
                ]
            }
        "#;

        let result = super::json_to_csp(json);

        let domains = result.unwrap(); 

        assert_eq!(domains.domains[0].domain, "example.com");
        assert_eq!(domains.domains[1].domain, "test.com");
        assert!(domains.domains[1].directive.script_src);
    }

    #[test]
    fn test_json_to_csp_empty() {
        let json = r#""#;

        let domains = super::json_to_csp(json);

        assert!(domains.is_none());
    }
}