enum IpAddrKind {
    V4,
    V6,
}

// enums can contain variants of different types for example-
enum IpAddrVals {
    V4(u8, u8, u8, u8),
    V6(String),
    // you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum!
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let ipv6 = IpAddrVals::V6(String::from("::1"));
    let ipv4 = IpAddrVals::V4(127, 0, 0, 1);
    let some_number = Some(5); // this has a type Option<i32>
    let some_string = Some("a string"); // Option<&str>
    let absent_number: Option<i32> = None;
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // here other is just a variable name, which is passed to move_player, with this we have defined the exhaustive list without defining exhaustively for each pattern.
        // This is called catch-all arm and should be present at last, since match is like a waterfall pattern, and falls into the first pattern big enough for that waterfall.
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // if we don't want to use the value, if we have used other and still don't want to pass the value, it's fine. It's just not recommended.
        // _ => (),  this also works if you don't wanna do anything on the dice value other than 3 or 7.
    }
    // if let takes a pattern and an expression separated by an equal sign.
    let config_max=Some(5);
    if let Some(max)=config_max{
        println!("max is configured to be {}", max);
    }
}

fn route(ip_kind: IpAddrKind) {}

impl IpAddrVals {
    fn isLoopBack(&self) {}
}

/*
 * this is how std lib stores ipv4 and ipv6
 */
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
/*
// Options Enum from stdlib- The Option type encodes the very common scenario in which a value could be something or it could be nothing.
enum Option<T> {
    None,
    Some(T),
}
*/

// The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

fn value_in_cents(coin: Coin) -> u8 {
    // with if, the expression needs to return a Boolean value, but with match, it can return any type.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => {
            println!("Quarter from {:?}", UsState);
            25
        }
    }
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}