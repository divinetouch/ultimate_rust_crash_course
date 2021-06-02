use std::thread;

fn main() {
    let add = |x, y| x + y;

    let result = add(1, 2);

    println!("{}", result);

    let s = "🍓".to_string();
    // move keyword mean move the ownership to the
    // closure so can be passed around (or between thread)
    let f = move || {
        println!("{}", s);
    };

    f();

    let v = vec![2, 4, 6];

    let acc = v
        .iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);

    println!("{:?}", acc);

    let handle = thread::spawn(move || {
        // this is the main function of the thread
    });

    // do stuff simultaneously in the main thread

    // wait untile thread has exited
    handle.join().unwrap();
}
