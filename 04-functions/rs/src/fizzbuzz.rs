pub fn demo() {
    println!("====");
    println!("fizzbuzz");
    println!("====");

    let fizzbuzz = |x| {
        if x % 15 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }
    };

    let fizzbuzz2 = |x: i32| {
        if x % 15 == 0 {
            return "FizzBuzz".to_string();
        } else if x % 3 == 0 {
            return "Fizz".to_string();
        } else if x % 5 == 0 {
            return "Buzz".to_string();
        } else {
            return x.to_string();
        }
    };
    for i in 1..42 {
        fizzbuzz(i)
    }
    println!("====");
    (1..42).into_iter().for_each(fizzbuzz);
    println!("====");
    println!("{:?}", (1..42).map(fizzbuzz2).collect::<Vec<String>>());
}
