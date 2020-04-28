use crate::directives::line;
use crate::domains::Collection;
use std::thread;
use std::thread::JoinHandle;

fn create(directive: String, domains: Collection) -> JoinHandle<String> {
    thread::spawn(move || line::build(directive, domains.clone()))
}

pub fn build_lines(
    directives: Vec<String>,
    domains: Collection,
) -> Vec<JoinHandle<String>> {
    let mut threads: Vec<JoinHandle<String>> = vec![];

    for directive in directives {
        threads.push(self::create(directive, domains.clone()));
    }

    threads
}

// -----
// Tests
// -----
#[cfg(test)]
mod threads_test {
    use crate::domains::{ Collection, Item };

    #[test]
    fn test_create() {
        let domain = String::from("*.google.com");
        let directive = vec![String::from("connect-src")];

        let domain = Item{ domain, directive };
        let domains: Collection = vec![domain];
        
        let directive_check = String::from("connect-src");

        let result = super::create(directive_check, domains);

        assert_eq!(
            result.join().unwrap().as_str(),
            "connect-src *.google.com; "
        );
    }
}
