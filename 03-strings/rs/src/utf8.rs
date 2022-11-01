pub fn demo() {

    println!("===");
    println!("Demo UTF8");
    println!("===");

    let s = String::from("Simpson");
    println!(
        "'{}': length: {} chars: {} memsize: {}",
        s,
        s.len(),
        s.chars().count(),
        std::mem::size_of::<String>() + s.len()
    );
    println!("{:?}", s.as_bytes());
    println!("after Si: {:?}", &s[2..]);

    println!("===");

    let unicode = String::from("Thanks 😊");
    println!(
        "'{}': length: {} chars: {} memsize: {}",
        unicode,
        unicode.len(),
        unicode.chars().count(),
        std::mem::size_of::<String>() + unicode.len()
    );
    println!("{:?}", unicode.as_bytes());
    println!("after Th: {:?}", &unicode[2..]);
}