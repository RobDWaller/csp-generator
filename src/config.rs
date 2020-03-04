use crate::GetDirectives;

pub struct Directives {
    list: Vec<String>
}

impl GetDirectives for Directives {
    fn get_directives(&self) -> Vec<String> {
        self.list.clone()
    }
}

pub fn get_directives() -> Directives {
    Directives{
        list: vec![
            String::from("script-src"),
            String::from("connect-src")
        ]
    }
}

#[cfg(test)]
mod config_test {
    use super::GetDirectives;

    #[test]
    fn test_get_directives() {
        let config: super::Directives = super::get_directives();

        assert_eq!(config.get_directives()[0], String::from("script-src"));
    }
}
