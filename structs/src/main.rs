enum IpAddressType {
    V4,
    V6,
}

fn main() {
    let ip_type = IpAddressType::V4;
    let ip_type_v6 = IpAddressType::V6;

    match ip_type {
        IpAddressType::V4 => println!("IPv4 address"),
        IpAddressType::V6 => println!("IPv6 address"),
    }

    match ip_type_v6 {
        IpAddressType::V4 => println!("IPv4 address"),
        IpAddressType::V6 => println!("IPv6 address"),
    }

    enum State {
        California,
        Texas,
        Florida,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(State),
    }

    match Coin::Quarter(State::California) {
        Coin::Penny => println!("This is a penny."),
        Coin::Nickel => println!("This is a nickel."),
        Coin::Dime => println!("This is a dime."),
        Coin::Quarter(state) => match state {
            State::California => println!("This is a quarter from California."),
            State::Texas => println!("This is a quarter from Texas."),
            State::Florida => println!("This is a quarter from Florida."),
        },
    }

    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    };

    if let Some(5) = x {
        println!("yay! matched");
    }

    x
}
