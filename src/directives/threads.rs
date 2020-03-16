use std::thread;
use std::thread::JoinHandle;
use crate::domains;
use crate::directives::line;

fn create_thread(directive: String, domains: domains::Collection) -> JoinHandle<String> {
    thread::spawn(move || line::build(directive, domains.clone()))
}

pub fn build_lines(directives: Vec<String>, domains: domains::Collection) -> Vec<JoinHandle<String>> {
    let mut threads: Vec<JoinHandle<String>> = vec![];

    for directive in directives {
        threads.push(self::create_thread(directive, domains.clone()));
    }

    threads
}

#[cfg(test)]
mod threads_test {

}