pub fn demo() {
    println!("====");
    println!("functions");
    println!("====");

    println!("{}", outer(2));
}

fn outer(x: u32) -> u32 {
    fn inner(a: u32) -> u32 {
        a + 40
    }
    inner(x)
}
