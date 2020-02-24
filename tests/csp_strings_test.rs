#[cfg(test)]
mod csp_string_test {
    extern crate csp_generator;

    use csp_generator::csp_json;
    use csp_generator::csp_item;
    use csp_generator::csp_string;

    #[test]
    fn test_directive_line() {

        let directive: Vec<String> = vec![
            String::from("connect-src"),
            String::from("script-src")
        ];

        let item = csp_item::CspItem{
            domain: String::from("*.example.com"),
            directive: directive
        };

        let mut domains: Vec<csp_item::CspItem> = Vec::new();
        domains.push(item);

        let json = csp_json::CspJson{
            domains: domains
        };

        let connect_src: String = csp_string::directive_line("connect-src", json);

        assert_eq!(connect_src, String::from("connect-src: *.example.com;"));
    }
}