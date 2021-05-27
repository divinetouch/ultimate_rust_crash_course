fn main() {
    let s1 = String::from("abc");
    let s2 = s1; // can't use s1 anymore -> move s1 to s2
    println!("Hello, world!, {}", s2);

    // if we want to keep using s2
    // clone is deep copy, include heap and pointer data
    // copy is when only stack data is being copy
    let mut s3 = s2.clone();
    println!("Hello, world!, {}, {}", s2, s3);

    do_stuff(&s3);
    do_mut_stuff(&mut s3);
}

// borrow a reference
fn do_stuff(s: &String) {
    println!("{}", s);
}

fn do_mut_stuff(s: &mut String) {
    (*s).insert_str(0, "Hi, "); // de reference is low president => need parenthesis
                                // s.insert_str(0, "Hi, "); // dot notation doesn't need de referencing
    println!("{}", s);

    // de-referencing s to replace the whole value of s
    *s = String::from("Replacement");

    println!("{}", s);
}
