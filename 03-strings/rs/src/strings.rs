pub fn demo() {

    println!("===");
    println!("Demo strings");
    println!("===");

    let homer = String::from("I want a donut");
    let marge: String = homer.replace("donut", "kiss");

    println!("Homer says: {}", homer);
    println!("Marge replies: {}", marge);
    println!("Marge adds: {} please", marge);

    let phrase_homer: &'static str = "trying is the first step towards failure";
    println!("Homer's quote: {}", phrase_homer);

    println!("Words in reverse");
    for word in phrase_homer.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = phrase_homer.chars().collect();
    chars.sort();
    chars.dedup();

    let mut s = String::new();
    for c in chars {
        s.push(c);
        s.push_str(", ");
    }

    let trim: &[char] = &[' ', ','];
    let trimmed_str: &str = s.trim_matches(trim);
    println!("Used    characters: {}", s);
    println!("Trimmed characters: {}", trimmed_str);
}