[![Actions Status](https://github.com/RobDWaller/csp-generator/workflows/Build%20and%20Test/badge.svg)](https://github.com/RobDWaller/csp-generator/actions) [![Crates.io](https://img.shields.io/crates/v/csp_generator)](https://crates.io/crates/csp_generator) [![codecov](https://codecov.io/gh/RobDWaller/csp-generator/branch/master/graph/badge.svg)](https://codecov.io/gh/RobDWaller/csp-generator)
# Content Security Policies Generator

Managing and creating Content Security Policies can be a challenge. The Content Security Policy header format does not lend itself to managing lots of domains across multiple directives. Especially if you need to allow Google Analytics.   

This Rust library allows you to generate a CSP header string from well organised JSON strings. The JSON structure this library accepts allows you to more easily manage many domains and many directives for your website CSP policies.

If you need to learn more about Content Security Policies we suggest you read the following resources:

- [Content Security Policies Website](https://content-security-policy.com/)
- [MDN Content Security Policies Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
- [RFC 7762](https://tools.ietf.org/html/rfc7762)

## Installation 

To install csp_generator in your Rust project simply add it as a dependency within your Cargo manifest.

**Example Cargo.toml**
```toml
[dependencies]
csp_generator = "0.2.0-beta.3"
```

## Usage

This library exposes three methods:

- `csp_generator::enforce()`
- `csp_generator::report_only()`
- `csp_generator::csp_only()`

The `enforce()` and `report_only()` methods will return a struct which contains a header string and a csp string. This will make sure you have the correct CSP header and CSP directive string dependent on whether you wish to use enforcement or report only modes.

If you only wish to return the CSP directive string call the `csp_only()` method.

Each of the methods accepts two arguments a list of CSP directives you wish to use, plus the JSON config. You can use the built in CSP directives list configuration if you wish as it contains all the standard CSP directives, see `csp_generator::config`. However, as this functionality complies with an interface, it can be overridden.

**Example Code**

```rust
use csp_generator::directives;

let json = r#"
    [
        {"domain": "example.com", "directive": ["connect-src"]},
        {"domain": "test.com", "directive": ["connect-src", "script-src"]}
    ]
"#;

let csp: String = csp_generator::enforce(directives::get_directives(), json);

println!("This is the CSP Header: {}", csp.header);
// This is the CSP Header: Content-Security-Policy
println!("This is the CSP Directives String: {}", csp.csp);
// This is the CSP Directives String: script-src test.com; connect-src example.com test.com;
```

## JSON Configuration

This library relies on a specific JSON format. This is an array of objects which contain two properties. The `domain` which is a string and the `directive` which is an array of directive strings.

**Format**
```js
[
    {"domain": string, "directive": array<string>}
]
```

**Example Config**
In this example we associate example.com with the connect-src directive and the test.com domain with the connect-src and script-src directives.

```js
[
    {"domain": "example.com", "directive": ["connect-src"]},
    {"domain": "test.com", "directive": ["connect-src", "script-src"]}
]
```

## CSP Directives List

Along with supplying a list of domains and directives in JSON format, we also need to supply the csp_generator with a list of directives which we want to use in our CSP. 

You can use the built CSP directives list config, as it contains a list of all the standard CSP directives. But if you wish to override this you can.

You just need to comply with the `csp_generator::directives::GetDirectives` trait (interface).

**Example Override**
This override will generate a CSP directive string which only makes use of the script-src and connect-src. 

```rust
use crate::GetDirectives;

pub struct MyDirectives {
    list: Vec<String>,
}

impl GetDirectives for MyDirectives {
    fn get_directives(&self) -> Vec<String> {
        self.list.clone()
    }
}

// Construct MyDirectives Struct with the directives you wish to use.
pub fn my_directives() -> Directives {
    MyDirectives {
        list: vec![
            String::from("script-src"),
            String::from("connect-src"),
        ],
    }
}
```

## License
MIT

## Author
[@RobDWaller](https://twitter.com/RobDWaller)