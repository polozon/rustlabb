
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
        username: String::from("Peter"),
        email: String::from("nobody@none.com"),
        sign_in_count: 1,
    };
    println!("user1.username = {}", user1.username);
    println!("is user Peter = {}", user1.is_user("Peter"));
}

fn analyze_slice(slice: &[i32]) {   // Borrowed slice
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn testing_slices() {
    println!("---- Testing slices ----");
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    analyze_slice(&xs[1 .. 3]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}

impl User {
    fn is_user(self, username: &str) -> bool {
        self.username == username
    }
}

fn main() {
    println!("Hello!");
    let ret: String = test_tuple();
    println!("ret = {}", ret);

    test_struct();
    println!("1 << 5 is {}", 1u32 << 5);

    testing_slices();
}
