use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync:: {Mutex, Arc};

fn main() {

    spawn_thread();

    messaging();
    
    mutex();
}


fn mutex() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];


    for _ in 0..10 {

        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}



fn messaging() {
    println!("-- message over thread ------------------------");

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),

        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //handle.join().unwrap();

    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),

        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //handle1.join().unwrap();

    for received in rx {
        println!("Got: {}", received);
    }

}


fn spawn_thread() {

    println!("-- handle thread ------------------------");
    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    println!("-- move ownership over thread ------------------------");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //drop(v);

    handle.join().unwrap();

}
