pub fn demo() {

    println!("===");
    println!("Demo multiline");
    println!("===");

    let multi = "here are
many lines
    including whitespaces"
        .to_string();
    println!("{}", multi);

    let escaped = " foo \"\"\" bar '
a new line
\n
nice
"
    .to_string();
    println!("{}", escaped);

    let single_char = 's';
    println!("{}", single_char);

}