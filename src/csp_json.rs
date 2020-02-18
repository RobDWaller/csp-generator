use crate::csp_item;
use crate::csp_directive;

fn 

pub fn json_to_csp(json: String) -> Vec<csp_item::CspItem> {
    let mut collection: Vec<csp_item::CspItem> = Vec::new();

    let directive = csp_directive::CspDirective{
        connect_src: true,
        script_src: false,
        img_src: false,
        style_src: true,
        object_src: true,
        frame_src: true,
        media_src: false,
        script_src_elem: true,
        worker_src: false
    };

    let directive_two = csp_directive::CspDirective{
        connect_src: true,
        script_src: false,
        img_src: false,
        style_src: true,
        object_src: true,
        frame_src: true,
        media_src: false,
        script_src_elem: true,
        worker_src: false
    };

    let item = csp_item::CspItem{
        domain: String::from("example.com"),
        directive: directive
    };

    let item_two = csp_item::CspItem{
        domain: String::from("test.com"),
        directive: directive_two
    };

    collection.push(item);
    collection.push(item_two);

    return collection;
}

#[cfg(test)]
mod csp_json_test {
    #[test]
    fn test_json_to_csp() {
        let json = r#"
            {
                "example.com": {true, false},
                "test.com": {false, false, true}
            }
        "#;

        let collection = super::json_to_csp(String::from(json));

        assert_eq!(collection[0].domain, "example.com");
        assert_eq!(collection[1].domain, "test.com");
    }
}