pub fn demo() {
    let mut v = vec![1_i32, 20, 13]; //all are i32
    println!("Initial vector: {:?}", v);

    println!("Push 42 into the vector");
    v.push(42);
    println!("Vector becomes : {:?}", v);

    println!("Vector length: {}", v.len());

    println!("2nd element: {}", v[1]);

    println!("Pop last element: {:?}", v.pop());

    println!("Iterating via borrowing");
    for e in &v {
        // borrowing the vector
        println!("found element {}", e);
    }

    println!("Iterating via iter()");
    for e in v.iter() {
        println!("found element {}", e);
    }

    println!("Iterating via enumerate()");
    for (i, e) in v.iter().enumerate() {
        println!("We found {} at index {} ", e, i);
    }

    println!("Mutating elements via iter_mut()");
    for e in v.iter_mut() {
        *e *= 2;
    }

    println!("Doubled vectot is now vector: {:?}", v);

    let v: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("now quadupled: {:?}", v);

    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);
}
