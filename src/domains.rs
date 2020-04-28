#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub domain: String,
    pub directive: Vec<String>,
}

pub type Collection = Vec<Item>;

// -----
// Tests
// -----
#[cfg(test)]
mod item_test {
    #[test]
    fn test_item_struct() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = super::Item {
            domain: String::from("*.example.com"),
            directive: directives,
        };

        assert_eq!(item.domain, "*.example.com");
        assert_eq!(item.directive[1], "script-src");
    }

    #[test]
    fn test_collection() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = super::Item {
            domain: String::from("*.example.com"),
            directive: directives,
        };

        let mut domains: super::Collection = vec![];
        domains.push(item);

        assert_eq!(domains[0].domain, "*.example.com");
        assert_eq!(domains[0].directive[1], "script-src");
    }
}
