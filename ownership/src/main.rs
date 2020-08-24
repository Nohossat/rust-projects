fn main() {
    let mut s1 = String::from("hello"); // requests memory for this text
    s1.push_str(", world");

    let s2 = s1; // in Rust, this is called a 'move'
    let mut s3 = s2.clone(); // do a deep copy of the data
    s3.push_str(", lord");
    println!("s1 = {}, s3 = {}", s2, s3);

    takes_owernship(s3);
    // println!("{}", s3); // shouldnt be working
    // Passing a variable to a function will move or copy

    let x = 5;
    let y = x;
    println!("x = {},  y = {}", x, y);
    makes_copy(x);
    println!("{}", x); // should be working since the type implement Copy

    let s4 = gives_ownership();
    println!("{}", s4);
    let s5 = String::from("bye");
    let s6 = takes_and_gives_back(s5);
    println!("{}", s6);

    let (mut s7, len) = calculate_length(s6);
    println!("s7 : {}, length of s7 : {}", s7, len);

    // how to use references
    let len2 = calculate_length_with_ref(&s7);
    println!("s7 : {}, length of s7 : {}", s7, len2); // we can still use s7

    // change(&s7); // references are immutable by default
    change_mutable_ref(&mut s7); // only 1 mutable ref possible in the same scope : NO DATA RACE, can't have an immutable var at the same time neither
    println!("s7 : {}", s7); 

    // let reference_to_nothing = dangle();
} // after the curly brace the variable s2, s6 are deallocated
// s1, s3, s5 have been moved (deactivated) so no need to do anything

fn takes_owernship(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("from gives_ownership");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // annoying to return the arguments and the resulting values each time
}

// how to use references
fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
}

/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change_mutable_ref(some_string: &mut String) {
    some_string.push_str(", salut toi");
}

/* dangling reference
fn dangle() -> &String {
    let s = String::from("salut");

    &s // dangling reference since the s object will be dropped after the curly brace
    // return the String instead
}
*/
