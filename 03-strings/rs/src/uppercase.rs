fn capitalize1(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

fn capitalize2(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
    }
}

pub fn demo() {

    println!("===");
    println!("Demo uppercase");
    println!("===");

    let names: Vec<String> = vec![
        "homer".to_string(),
        "marge".to_string(),
        "bart".to_string(),
        "9lisa".to_string(),
        "".to_string(),
        "maggie".to_string(),
    ];
    for name in names.iter() {
        println!("{} -> {}", name, capitalize1(name));
        println!("{} -> {}", name, capitalize2(name));
        println!("");
    }
}