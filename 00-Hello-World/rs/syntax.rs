//! docstring for module
//!
//! Compile it with :  rustc syntax.rs
//! Run it : ./syntax

/// docstring for function
fn substract(x: i64, y: i64) -> i64 {
    // one line comment
    x - y
    /* multiline comment explaining
     * that for returning a value
     * an expression as last line is enough
     * (note the absence of ;)
     */
}

fn main() {
    let z = substract(48, 6);
    println!("result of substract : {}", z);
    let txt = format!("result of substract : {z}", z = z);
    println!("{}", txt);
}
