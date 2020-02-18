use crate::csp_json;

fn connect_src(csp: csp_json::CspJson) -> String {
    let mut connect_src = String::from("connect-src:");

    for domain in csp.domains {
        connect_src.push_str(" ");
        connect_src.push_str(domain.domain.as_str());
    }

    connect_src.push_str(";");
    return connect_src;
}

#[cfg(test)]
mod csp_string_test {
    use crate::csp_item;
    use crate::csp_directive;
    use crate::csp_json;

    #[test]
    fn test_connect_src() {
        let directive = csp_directive::CspDirective{
            connect_src: true,
            script_src: false,
            img_src: true,
            style_src: false,
            object_src: true,
            frame_src: false,
            media_src: true,
            script_src_elem: false,
            worker_src: true
        };

        let item = csp_item::CspItem{
            domain: String::from("*.example.com"),
            directive: directive
        };

        let mut domains: Vec<csp_item::CspItem> = Vec::new();
        domains.push(item);

        let json = csp_json::CspJson{
            domains: domains
        };

        let connect_src: String = super::connect_src(json);

        assert_eq!(connect_src, String::from("connect-src: *.example.com;"));
    }
}