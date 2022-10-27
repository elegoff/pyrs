pub fn demo() {
    let multi_types = (1u8, "nce", "upon", 'a', true, 3.1416_f32);

    println!("first value: {}", multi_types.0);
    println!("second value: {}", multi_types.1);

    let nested_tuple = ((4u8, 2i32, 6f32), (-1i32, 3i16, 2u8), (9u8, 3i16, 5f64));
    println!("nested_tuple: {:?}", nested_tuple);

    //destructuring / unpacking
    let (a, b, c, d, e, f) = multi_types;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);

    let (a2, (b2, c2)) = (0, (1, 2));

    println!("{}", a2);
    println!("{}", b2);
    println!("{}", c2);
}
