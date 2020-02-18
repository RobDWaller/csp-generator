pub struct CspDirective{
    pub connect_src: bool,
    pub script_src: bool,
    pub img_src: bool,
    pub style_src: bool,
    pub object_src: bool,
    pub frame_src: bool,
    pub media_src: bool,
    pub script_src_elem: bool,
    pub worker_src: bool
}

#[cfg(test)]
mod csp_directive_test {
    #[test]
    fn test_csp_item_struct() {
        let directive = super::CspDirective{
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

        assert_eq!(directive.connect_src, true);
        assert_eq!(directive.script_src, false);
        assert_eq!(directive.img_src, false);
        assert_eq!(directive.style_src, true);
        assert_eq!(directive.object_src, true);
        assert_eq!(directive.frame_src, true);
        assert_eq!(directive.media_src, false);
        assert_eq!(directive.script_src_elem, true);
        assert_eq!(directive.worker_src, false);
    }
}