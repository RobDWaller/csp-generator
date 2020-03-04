use serde_json::error;
use crate::domains;

pub fn json(json: &str) -> Result<domains::Collection, error::Error> {    
    // let result = serde_json::from_str(json);

    // if result.is_ok() {
    //     let parsed: domains::Collection = result.unwrap();

    //     return Ok(parsed);
    // }

    // Err(String::from("Unable to parse JSON."))

    // let parsed: domains::Collection = serde_json::from_str(json)?.unwrap();
    // Ok(parsed)

    let result = serde_json::from_str(json);    

    match result {
        Ok(result) => { 
            let parsed: domains::Collection = result;
            Ok(parsed)
        },
        Err(e) => Err(e)
    }
}

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

        let result = super::json(json);

        let domains = result.unwrap(); 

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
}