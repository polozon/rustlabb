# Rustlab

Labbar med Rust.

## Skapade projektet

```bash
cargo new rustlabb
cd rustlabb
touch README.md
code .
```

Publish to github direkt från vscode.

## Studiemedel

- Läser från [rust boken](https://doc.rust-lang.org/book/title-page.html). 
- Kolla också in [rust by example](https://doc.rust-lang.org/rust-by-example/index.html).


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