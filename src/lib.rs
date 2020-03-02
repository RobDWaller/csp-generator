#[macro_use]
extern crate serde_derive;

mod domains;
mod parse;
pub mod directives;
pub mod config;

pub trait GetDirectives {
    fn get_directives(&self) -> Vec<String>;
}