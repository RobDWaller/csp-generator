#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub domain: String,
    pub directives: Vec<String>,
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
            directives,
        };

        assert_eq!(item.domain, "*.example.com");
        assert_eq!(item.directives[1], "script-src");
    }

    #[test]
    fn test_collection() {
        let directives: Vec<String> = vec![String::from("connect-src"), String::from("script-src")];

        let item = super::Item {
            domain: String::from("*.example.com"),
            directives,
        };

        let mut domains: super::Collection = vec![];
        domains.push(item);

        assert_eq!(domains[0].domain, "*.example.com");
        assert_eq!(domains[0].directives[1], "script-src");
    }
}
