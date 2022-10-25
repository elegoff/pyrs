pub fn using() {
    let a = 42;
    println!("a: {}", a);
    //shadowing the value
    let a = "forty two".to_string();
    println!("a: {}", a);

    // declare, initialize
    let something: Option<u32> = None;
    let x = 5;
    println!("x, something: {}, {:?}", x, something);
    let something = Some(x * 5);
    println!("x, something: {}, {:?}", x, something);

    // Mutability
    let mut y = 0;
    y = y * 2 + x;
    dbg!(y);

    const FOO: i32 = 42;
    let y = y * FOO;
    dbg!(y);
}
