use crate::GetDirectives;

pub struct Directives {
    list: Vec<String>,
}

impl GetDirectives for Directives {
    fn get_directives(&self) -> Vec<String> {
        self.list.clone()
    }
}

pub fn get_directives() -> Directives {
    Directives {
        list: vec![
            String::from("default-src"),
            String::from("script-src"),
            String::from("style-src"),
            String::from("img-src"),
            String::from("connect-src"),
            String::from("font-src"),
            String::from("object-src"),
            String::from("media-src"),
            String::from("frame-src"),
            String::from("sandbox"),
            String::from("report-uri"),
            String::from("child-src"),
            String::from("form-action"),
            String::from("frame-ancestors"),
            String::from("plugin-types"),
            String::from("report-to"),
            String::from("worker-src"),
            String::from("manifest-src"),
            String::from("navigate-to"),
        ],
    }
}

#[cfg(test)]
mod config_test {
    use super::GetDirectives;

    #[test]
    fn test_get_directives() {
        let config: super::Directives = super::get_directives();

        assert_eq!(config.get_directives()[0], String::from("default-src"));
        assert_eq!(config.get_directives()[9], String::from("sandbox"));
        assert_eq!(config.get_directives()[18], String::from("navigate-to"));
    }
}
