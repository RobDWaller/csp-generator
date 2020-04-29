use serde_json;

#[derive(Deserialize, Serialize, Clone)]
pub struct Item {
    pub domain: String,
    pub directives: Vec<String>,
}

pub type Collection = Vec<Item>;

pub trait ToJson {
    fn to_json(&self) -> String;
}

impl ToJson for Collection {
    fn to_json(&self) -> String {
        let mut json = "[".to_string();

        for item in self {
            json.push_str(serde_json::to_string(&item).unwrap().as_str());
        }

        json.push_str("]");

        json
    }
}

// -----
// Tests
// -----
#[cfg(test)]
mod item_test {
    use super::{Collection, Item, ToJson};

    #[test]
    fn test_item_struct() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = Item {
            domain: String::from("*.example.com"),
            directives,
        };

        assert_eq!(item.domain, "*.example.com");
        assert_eq!(item.directives[1], "script-src");
    }

    #[test]
    fn test_collection() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = Item {
            domain: String::from("*.example.com"),
            directives,
        };

        let mut domains: Collection = Collection::new();
        domains.push(item);

        assert_eq!(domains[0].domain, "*.example.com");
        assert_eq!(domains[0].directives[1], "script-src");
    }

    #[test]
    fn test_to_json() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = super::Item {
            domain: String::from("*.example.com"),
            directives,
        };

        let mut domains: Collection = Collection::new();
        domains.push(item);

        assert_eq!(
            domains.to_json(),
            r#"[{"domain":"*.example.com","directives":["connect-src","script-src"]}]"#
        );
    }
}
