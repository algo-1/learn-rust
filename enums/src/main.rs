struct Ipv4Addr(String);
struct Ipv6Addr(String);

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4(val) => println!("This is a V4 Ip Address, value = {}", val.0),
            IpAddr::V6(val) => println!("This is a V6 Ip Address, value = {}", val.0)
        }
    }
}

fn minus_one(num: Option<isize>) -> Option<isize> {
    match num {
        None => None,
        Some(val) => {
            println!("{} minus 1 = {}", val, val - 1 );
            Some(val - 1)
        }
    }
}

fn print_number(num: Option<isize>) {
    match num {
        None => println!("value = None"),
        Some(val) => println!("value = {}", val)
    }
}

fn print_number_u8(num: Option<u8>) {
    match num {
        None => println!("value = None"),
        Some(val) => println!("value = {}", val)
    }
}

fn main() {
    let home = Ipv4Addr(String::from("127.0.0.1"));
    let loopback = Ipv6Addr(String::from("::1"));

    let home = IpAddr::V4(home);
    let loopback = IpAddr::V6(loopback);
    
    home.print();
    loopback.print();

    let three = Some(3);
    let two = minus_one(three);
    let one = minus_one(two);
    let none = minus_one(None);

    print_number(three);
    print_number(two);
    print_number(one);
    print_number(none);

    let some_u8_number= Some(100u8);
    if let Some(100) = some_u8_number {
        print_number_u8(some_u8_number);
    } // no exhaustive checking (add else for that) but tbh 'match' looks way better, perhaps there'll be a case where it's more convenient to use the if let syntax  

    match some_u8_number {
        Some(100) => print_number_u8(some_u8_number),
        _ => ()  // None & all other possible u8 numbers ([0 - 255] except 100)
    }
}
