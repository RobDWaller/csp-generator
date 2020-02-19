//use crate::csp_directive;

#[derive(Serialize, Deserialize)]
pub struct CspItem{
    pub domain: String,
    //pub directive: csp_directive::CspDirective
    pub directive: Vec<String>
}

#[cfg(test)]
mod csp_item_test {
    #[test]
    fn test_csp_item_struct() {
        let directive: Vec<String> = vec![
            String::from("connect-src"),
            String::from("script-src")
        ];

        let item = super::CspItem{
            domain: String::from("*.example.com"),
            directive: directive
        };

        assert_eq!(item.domain, "*.example.com");
        assert_eq!(item.directive[1], "script-src");
    }
}
