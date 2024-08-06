
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn test_tuple() -> String {
    let mytuple: (i32, u8, &str) = (1, 2, "hej");
    let (a, b, c) = mytuple;
    
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("mytuple.2 = {}", mytuple.2);
    // Returning a string, no semicolon at the end
    mytuple.2.to_string()
}

fn test_struct() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("nobody@none.com"),
        sign_in_count: 1,
    };
    println!("user1.username = {}", user1.username);
}

fn main() {
    println!("Hello!");
    let ret: String = test_tuple();
    println!("ret = {}", ret);

    test_struct();
}
