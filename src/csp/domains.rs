use crate::domains::{Collection};

// Find the domains associated with the CSP directive
pub fn find(directive: &str, domains: &Collection) -> Vec<String> {
    let mut domains_found: Vec<String> = vec![];

    for domain in domains {
        if domain.directives.contains(&directive.to_string()) {
            domains_found.push(domain.domain.to_string());
        }
    }

    domains_found
}

#[cfg(test)]
mod domains_test {
    use crate::domains::{Collection, Item};

    #[test]
    fn test_find()
    {
        let item1 = Item {domain: "foo".to_string(), directives: vec!["img-src".to_string()]};
        let item2 = Item {domain: "bar".to_string(), directives: vec!["img-src".to_string()]};

        let domains: Collection = vec![item1, item2];

        let directive = "img-src";

        let result: Vec<String> = super::find(&directive, &domains);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "foo".to_string());
        assert_eq!(result[1], "bar".to_string());
    }

    #[test]
    fn test_find_one()
    {
        let item1 = Item {domain: "foo".to_string(), directives: vec!["script-src".to_string()]};
        let item2 = Item {domain: "bar".to_string(), directives: vec!["img-src".to_string()]};

        let domains: Collection = vec![item1, item2];

        let directive = "script-src";

        let result: Vec<String> = super::find(&directive, &domains);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "foo".to_string());
    }

    #[test]
    fn test_find_none()
    {
        let item1 = Item {domain: "foo".to_string(), directives: vec!["script-src".to_string()]};
        let item2 = Item {domain: "bar".to_string(), directives: vec!["img-src".to_string()]};

        let domains: Collection = vec![item1, item2];

        let directive = "connect-src";

        let result: Vec<String> = super::find(&directive, &domains);

        assert!(result.is_empty());
    }
}