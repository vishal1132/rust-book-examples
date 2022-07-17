struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // tuple structs
struct Point(i32, i32, i32); // tuple structs

fn build_user(email: String, username: String) -> User {
    User {
        email, // since the field name and the assigning variable name is exactly same.
        username, // same as above
        active: true,
        sign_in_count: 1,
    }
}
struct AlwaysEqual; // unit-like struct

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    /*
        let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
        };
        // this is one way to do it.
        // The other way would be to just modify the piece you want to and copy the rest from the other struct which is right after this block.
    */
    
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // this should come last
    };

    let black= Color(0, 0, 0);
    let origin= Point(0, 0, 0);

    let something=AlwaysEqual;

    let rect1 = Rectangle {
        width: dbg!(30), // dbg! can be put infront of any expression
        height: 50,
    };

    dbg!(&rect1); // to stdout debug information for ex- line number, file name etc.
    println!("{:#?}", rect1); 
    // println!("{}", rect1);  // this won't work since Rectangle doesn't implement Display()
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("{}",rect1.area())
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // this is just an assoicated method like `String::from()`, so something Like Rectangle::square(1)
    fn square(size: u32) -> Rectangle { 
        Rectangle {
            width: size,
            height: size,
        }
    }
}