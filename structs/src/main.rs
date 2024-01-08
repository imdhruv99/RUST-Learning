#[derive(Debug)]  // basic implementation of debug trait
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,

}

impl Rectangle {
    // method has self keyword
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

impl Rectangle {
    // associated function, associated function does not have self keyword
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn build_user(email:String, username:String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user_one = User{
        username: String::from("Dhruv Prajapati"),
        email: String::from("dhruv@dhruv.com"),
        sign_in_count: 3,
        active: true,
    };

    user_one.email = String::from("abcd@gmail.com");
    println!("User One struct is: {:#?}", user_one);

    let user_two = build_user(
        String::from("sanket@gmail.com"), 
        String::from("Sanket Patel"),
    );
    println!("User Two struct is: {:#?}", user_two);
    

    let user_three = User {
        username: String::from("Hetvi Soni"),
        email: String::from("hetvi@gmail.com"),
        ..user_one
    };
    println!("User Three struct is: {:#?}", user_three);

    // tuple structs
    // see both have same field type with different type
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // rectangle area counter
    let rectangle_one = Rectangle {
        width: 300,
        height: 500
    };
    println!("The Area of Rectangle is {} square pixels.", rectangle_one.area());

    // Method example with multiple argument
    let rectangle_two = Rectangle {
        width: 10,
        height: 20
    };

    let rectangle_three = Rectangle {
        width: 30,
        height: 40
    };

    println!("The Area of Rectangle is {} square pixels.", rectangle_three.can_hold(&rectangle_two));
    println!("The Area of Rectangle is {} square pixels.", rectangle_two.can_hold(&rectangle_three));

    // Associate Function example
    let rectangle_four = Rectangle:: square(20);
    println!("The size of Rectangle is {:#?}.", rectangle_four);
}
