// Directives config maintains the list of valid Content Security Policy 
// directives which can be used with this library by default. This can be
// overwritten using the supplied GetDirectives trait if required.
//
// See: https://content-security-policy.com/

pub struct Directives {
    list: Vec<String>,
}

// Trait / Interface allows this directives config to be overwritten with custom 
// setup if required.
pub trait GetDirectives {
    fn get_directives(&self) -> Vec<String>;
}

impl GetDirectives for Directives {
    fn get_directives(&self) -> Vec<String> {
        self.list.clone()
    }
}

// Named constructor for directives module, injects the valid CSP directives 
// into the Directives struct.
pub fn directives() -> Directives {
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
            String::from("child-src"),
            String::from("form-action"),
            String::from("frame-ancestors"),
            String::from("plugin-types"),
            String::from("report-to"),
            String::from("worker-src"),
            String::from("manifest-src"),
            String::from("navigate-to"),
            String::from("report-uri"),
        ],
    }
}

// -----
// Tests
// -----
#[cfg(test)]
mod config_test {
    use super::GetDirectives;

    #[test]
    fn test_get_directives() {
        let config: super::Directives = super::directives();

        assert_eq!(config.get_directives()[0], String::from("default-src"));
        assert_eq!(config.get_directives()[9], String::from("sandbox"));
        assert_eq!(config.get_directives()[18], String::from("report-uri"));
        assert_eq!(config.get_directives().len(), 19);
    }
}
