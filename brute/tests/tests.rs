mod localnet {
    use brute::localnet;

    #[test]
    #[allow(unused)]
    pub fn get_ip_stem_return() { //test that it returns the right thing
        let x = localnet::get_ip_stem(11);

        let home = String::from("192.168.1.11:22"); // I want to be able to test wherever I am.
        let school1 = String::from("10.200.0.11:22");
        let school2 = String::from("10.200.1.11:22");

        match x.ip_string {
            home => assert!(true),
            school1 => assert!(true),
            school2 => assert!(true),
            _ => assert_eq!(2, 2+1),
        }
    }

    #[test]
    #[allow(unused)]
    pub fn get_ip_stem_overflow() { // test that it cannot go out of bounds ie: 192.168.0.302
        #[allow(overflowing_literals)]
        let x = localnet::get_ip_stem(255);

        let home = String::from("192.168.1.0:22"); // I want to be able to test wherever I am.
        let school1 = String::from("10.200.0.0:22"); //school has multiple routers
        let school2 = String::from("10.200.1.0:22");

        match x.ip_string {
            home => assert!(true),
            school1 => assert!(true),
            school2 => assert!(true),
            _ => assert_eq!(2, 2+1),
        }
    }
}
