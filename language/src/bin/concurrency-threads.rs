use std::thread;
use std::time::Duration;

fn spawning_threads() {

    // Note that with this function, the new thread will be stopped when the main thread ends,
    // whether or not it has finished running
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

fn join_handles() {
    // A JoinHandle is an owned value that, when we call the join method on it, will wait for
    // its thread to finish
    let test_str = "how long am i";
    let handle = thread::spawn(move || sim_comp(test_str));

    let res = handle.join().unwrap();
    println!("the answer is {}", &res);
}

fn sim_comp(val: &str) -> usize {
    println!("in sim comp");
    thread::sleep(Duration::from_millis(2000));
    val.len()
}

fn closure_thread() {
    //let mut v = vec![String::from("A"), String::from("B"), String::from("C")];
    let mut v = vec!["A", "B", "C"];

    let handle = thread::spawn( move || {
        let s = String::from("KOOL");
        v.push( "ZZ" );
        println!("pushed s to v: {:?}", v);
    });
    handle.join().unwrap();
}

// fn closures() {
//    let s = String::from("BAM!!");
//    let mut v = vec!["A", "B", "C"];
//
//    let cls = move || {
//        //let mut v1 = v;
//        v.push( &s );
//        println!("Here's a String {} and vec {:?}", &s, v );
//    };
//    cls();
// }

fn main() {
    //spawning_threads();
    join_handles();
    //let s = String::from("WAM");
    //closure_thread();
}