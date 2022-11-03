pub fn demo() {
    println!("====");
    println!("closures");
    println!("====");

    //closure parameter type is set by 1st caller
    let my_closure = |x| x;

    let s = my_closure(String::from("closure"));

    //let n = my_closure(42); => not working
    //since closure now expects a String param
    //need a conversion

    let n = my_closure(42.to_string());

    println!("{} {}", s, n);

    let v = vec![1, 2, 3];

    let is_same = |z| z == v;

    println!("Comparing to {:?}", v);
    let v2 = vec![1, 2, 3];
    assert!(is_same(v2));

    let things: Vec<i32> = (1..42).collect();
    let doubling: Vec<_> = things.iter().map(|x| x * 2).collect();
    let total: i32 = things.iter().map(|x| x + 1).sum();
    println!("{:?} {}", doubling, total);

    let multi_args = |a, b, c| (a + b) * c;
    println!("{}", multi_args(20, 1, 2));
}
