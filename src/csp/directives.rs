use crate::csp::directive;
use crate::domains::Collection;
use std::thread;
use std::thread::JoinHandle;

// Generate each directive based on the domains collection supplied and the
// directives config.
pub fn generate(directives_config: Vec<String>, domains: Collection) -> Vec<JoinHandle<String>> {
    let mut directives: Vec<JoinHandle<String>> = vec![];

    for directive_item in directives_config {
        let domains_clone = domains.clone();

        directives.push(
            thread::spawn(move || directive::generate(directive_item, domains_clone))
        );
    }

    directives
}

// -----
// Tests
// -----
#[cfg(test)]
mod threads_test {
}
