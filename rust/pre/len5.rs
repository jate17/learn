struct User {
    username: String,
    active: bool,
    age: u8
}

struct Point(i32, i32);
struct Empty;


fn main(){
    let user1 = User {
        username: String::from("Marco"),
        active: true,
        age: 29
    };

    println!("User: {} ({} anni) - Active: {}", user1.username, user1.age, user1.active );


    let p = Point(3,4);
    println!("Cordinate: {}, {}", p.0, p.1 );
    let _e = Empty; // -> Non ho capito a cosa serve una srttura vuota



    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    match home {
        IpAddr::V4(addr) => println!("IP.V4 {}", addr),
        IpAddr::V6(addr) => println!("IP.V6 {}", addr),

    }

}
