use std::fs::File;
use std::io::Error;
use rand::Rng;
//use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

struct User {
    active: bool,
    username: String,
}

fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(pol) => pol, // pol could be any name
        Err(_) => 0,
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too big!"),
        std::cmp::Ordering::Equal => println!("You win!"),
    };
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
    };
    println!("user1.username = {}", user1.username);
    println!("user1.active = {}", user1.active);
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
            Some(xv) => println!("{}: {}", i, xv),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}

impl User {
    fn is_user(self, username: &str) -> bool {
        self.username == username
    }
}

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn test_failed_file_open() {
    let _f: Result<File, Error> = File::open("hello.txt");
    /*
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    */
}

fn loop_count() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };  // The semicolon ends the statement
    println!("The result is {}", result);
    result
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        print!("{number}! ");
    }
    println!("");
}

fn creates_string() -> String {
    let some_string = String::from("yours");
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
}

fn test_ownership() {
    let s1 = String::from("hello1");
    let s2 = s1;
    //println!("{s1}"); // This will not work
    println!("{s2}");

    let s1 = creates_string();
    let s2 = String::from("hello2");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}");
}

fn test_slice() {
    let s = String::from("hello world!");
    let hello = &s[0..5];
    let world = &s[6..];
    println!("{hello} - {world}");
}

fn test_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}

fn get_status(username: &str) -> Option<&str> {
    // some user lookup code here...
    if (username != "peter") {
        return None;
    }
    // if user exists, fetch their status and return that...
    Some("found_status")
}
  
fn test_option() {
    println!("Testing Option error handling");
    // now let's use that function
    let result = get_status("peter");
    match result {
        Some(status) => println!("{}", status),
        None => println!("couldn't find a status for peter"),
    }
}


fn main() {
    //guessing_game();
    let pol: i32 = loop_count();
    println!("The result returned is {pol}");

    let test1 : i32 = loop {
        if true {
            break 3; // ()
        }
    };
    println!("test1 = {}", test1);
 
    println!("\nHello!");
    let ret: String = test_tuple();
    println!("ret = {}", ret);

    test_struct();
    println!("1 << 5 is {}", 1u32 << 5);

    testing_slices();

    let test_panic = false;
    drink("water");
    if test_panic {
        drink("lemonade"); // This will panic    
    }
    drink("beer");

    test_failed_file_open();

    for_loop();
    test_ownership();
    test_slice();
    test_enum();
    test_option();

}
