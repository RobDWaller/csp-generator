use crate::csp_directive;

pub struct CspItem{
    pub domain: String,
    pub directive: csp_directive::CspDirective
}

#[cfg(test)]
mod csp_item_test {
    #[test]
    fn test_csp_item_struct() {
        let directive = super::csp_directive::CspDirective{
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

        let item = super::CspItem{
            domain: String::from("*.example.com"),
            directive: directive
        };

        assert_eq!(item.domain, "*.example.com");
        assert!(item.directive.style_src);
    }
}
