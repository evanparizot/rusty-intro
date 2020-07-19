use std::net::{IpAddr, Ipv4Addr};
fn main() {
    
    // fn route(ip_kind: IpAddrKind) {}
    
    // route(IpAddrKind::V4);
    
    // enum IpAddrKind {
    //     V4, V6
    // }
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String
    // }
    
    // let home = IpAddr { 
        //     kind: IpAddrKind::V4,
        //     address: String::from("127.0.0.1")
        // };
        
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127,0,0,1));

    // enum IpAddr {
    //     V4(String),
    //     V6(String)
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));


    enum Message {
        Quit,
        Move { x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ =>  ()
    }
}
