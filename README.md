# Content Security Policies Generator

This is a simple library which consumes a JSON formatted list of domains and CSP directives and outputs a correctly formatted Content Security Policy string.

# Usage

```rust
extern crate csp_generator;
use csp_generator::config;

let json = r#"
    {
        "domains": [
            {"domain": "example.com", "directive": ["connect-src"]},
            {"domain": "test.com", "directive": ["connect-src", "script-src"]}
        ]
    }
"#;

let csp: String = csp_generator::enforce(config::get_directives(), json);

println!("This is the CSP: {}", csp);
```