use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//fn main() {
//    let (tx, rx) = mpsc::channel();
//
//    thread::spawn(move || {
//        let val = String::from("hiya");
//        tx.send(val).unwrap();
//    });
//
//    // recv blocks until a value is sent down the channel, try_recv does not block
//    let received = rx.recv().unwrap();
//    println!("Got: {}", received);
//}

fn main() {
    let (tx, rx) = mpsc::channel();

    // cloning the transmitter in order to create multiple producers
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}