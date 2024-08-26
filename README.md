# Rustlabb

Ett labbprojekt för [Rust](https://www.rust-lang.org/).

## Studiemedel

- Snabb genomgång i [denna video](https://www.youtube.com/watch?v=br3GIIQeefY).
    - Se även [denna](https://www.youtube.com/watch?v=Z3xPIYHKSoI) från samma snubbe.
- Läser från [rust boken](https://doc.rust-lang.org/book/title-page.html). 
- Kolla också in [rust by example](https://doc.rust-lang.org/rust-by-example/index.html).
- [unwrap och expect](https://www.jacksondawkins.com/blog/unwrap-and-expect-in-rust)

## Skapade projektet

```bash
cargo new rustlabb
cd rustlabb
touch README.md
code .
```

Publish to github direkt från vscode.

Kör applikation med: `cargo run`.

## Test tuple

Funktionen testar tuples med värden av olika typer. 

## Return från funktion utan return

Funktionen [returnerar](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values) en string genom att utelämna semikolon på sista raden.
Man behöver inte använda return.

> You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly

## Macron

`println!("ret = {}", ret);` är ett exempel på ett macro, det ser man på semikolonet.

## Structs

[structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#defining-and-instantiating-structs) är det närmsta man kommer klasser. En metod läggs också till structen så här:

```rust
impl User {
    fn is_user(self, username: &str) -> bool {
        self.username == username
    }
}
```

Det fungerar trots att impl kommer senare i koden.

## Slices

Här testas slices, det som skickas till funktionen är `[2, 3]`

```rust
let xs: [i32; 5] = [1, 2, 3, 4, 5];
analyze_slice(&xs[1 .. 3]);
```

Med denna for-loop plockas elementen ut, loopen körs avsiktligen en gång för mycket men denna 'out of range' hanteras av koden.
Koden är tagen från [detta exempel](https://doc.rust-lang.org/rust-by-example/primitives/array.html)

```rust
for i in 0..xs.len() + 1 { // Oops, one element too far!
    match xs.get(i) {
        Some(xval) => println!("{}: {}", i, xval),
        None => println!("Slow down! {} is too far!", i),
    }
}
```

## Error handling

Kör detta från terminalen
`RUST_BACKTRACE=1 cargo run`

## Guessing game

La in kod från [rust boken](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html). Här demonstreras bland annat hur man lägger till en extern modul och hur enum Ordering används.

Men applikationen kraschar och avslöjar massor av info när man matar in ett icke-nummer, det fixas [här](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-invalid-input). Om man skriver icke-nummer blir det 0.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
};
```

## Semikolon efter break

Labbar med statement och expressions, det verkar inte spela någon roll om man har ett semikolon efter break expression, oklart varför, se [forum fråga](https://stackoverflow.com/questions/65024479/why-does-break-not-need-a-semicolon-when-ending-a-loop). Break verkar vara lite som return som inte heller behöver ett semikolon på slutet.

```rust
let _: i32 = loop {
    if true {
        break 3; // Semicolon or not, doesn't seem to matter
    }
};
```

## Loop labels

Man kan göra en break från en yttre loop med loop labels, se [detta avsnitt](https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops). Här används tick symbiolen som också används för lifetimes.

## Ownership

Läs [detta kapitel](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

> Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

En konstant sträng är en string literal (unmutable), inte att förväxla med String som använder heapminnet och är mutable. Om man kopierar en sträng till en annan variabel så blir den första variabeln ogiltig, detta för att undvika double free error på heap.

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}, world!");   // WILL FAIL !
```

En funktion kan skapa och returnera en sträng så här:

```rust
fn creates_string() -> String {
    let some_string = String::from("yours");
    some_string
}
```

Den anropande koden sparar normalt sett den skapande strängen och tar över ägandet. Se funktionen `test_ownership`.

> Assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable

För att inte behöva kopiera tillbaka varibler används referenser. Se [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html). Funktionen lånar strängen och kan inte ändra den innan den lämnas tillbaka, om man inte gör det till en mutable reference.

> A slice is a kind of reference, so it does not have ownership

## Enums

Man kan lägga in värden i enums vilket är lite coolt, dev debug trait kan man printa ut värdet.

```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
println!("{:?}", home);
```

## Felhantering

[Bra info om unwrap och expect](https://www.jacksondawkins.com/blog/unwrap-and-expect-in-rust). 

- Vill man manuellt hantera errors, använd Option eller Result.
- Option returnerar Some eller None.
- Result returnerar Ok eller Error
- Vill man panikstoppa app, använd unwarp eller expect.