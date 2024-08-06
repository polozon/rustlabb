
fn test_tuple() -> String {
    let mytuple: (i32, u8, &str) = (1, 2, "hej");
    let (a, b, c) = mytuple;
    
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("mytuple.2 = {}", mytuple.2);
    // Returning a string, no semicolon at the end
    mytuple.2.to_string()
}

fn main() {
    println!("Hello!");
    let ret: String = test_tuple();
    println!("ret = {}", ret);
}
