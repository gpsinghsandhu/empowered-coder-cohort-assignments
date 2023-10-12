mod broadcast;

use std::thread::{sleep, spawn};
use std::time::Duration;
use broadcast::{Broadcast, BroadcastService};

fn main() {
    println!("*******Case 1: create two receivers and try sending them messages*********");
    let (tx, service) = BroadcastService::<String>::create_channel();
    let rx1 = service.create_receiver();
    let rx2 = service.create_receiver();
    let mut handles = vec![];

    // sender thread
    let sender_handle = spawn(move || {
        for i in 0..3 {
            let message = format!("message{}", i);
            println!("Sender sending message: {}", message);
            tx.send(message).unwrap();
            sleep(Duration::from_secs(1));
        }
        println!("Sender closed");
    });
    handles.push(sender_handle);

    // receiver thread 1
    let handle = spawn(move || {
        for message in rx1 {
            println!("Receiver 1 received message: {}", message);
        }
        println!("Receiver 1 closed");
    });
    handles.push(handle);

    // receiver thread 1
    let handle2 = spawn(move || {
        for message in rx2 {
            println!("Receiver 2 received message: {}", message);
        }
        println!("Receiver 2 closed");
    });
    handles.push(handle2);

    for i in handles {
        i.join().unwrap();
    }

    println!("*********Case 2: reciever looses some message until reciever thread subscribes*********");

    let (tx2, service2) = BroadcastService::<String>::create_channel();

    // first sender thread
    let tx3 = tx2.clone();
    let mut sender_handle = spawn(move || {
        for i in 0..2 {
            let message = format!("message{}", i);
            println!("Case 2 Sender 1 sending message: {}, should be lost", message);
            tx3.send(message).unwrap();
            sleep(Duration::from_secs(1));
        }
        println!("Case 2 Sender 1 closed");
    });
    // wait for sender thread, all these messages should are consumed by broadcast thread
    sender_handle.join().unwrap();

    let rx3 = service2.create_receiver();

    sender_handle = spawn(move || {
        for i in 0..2 {
            let message = format!("message{}", i);
            println!("Case 2 Sender 2 sending message: {}", message);
            tx2.send(message).unwrap();
            sleep(Duration::from_secs(1));
        }
        println!("Case 2 Sender 2 closed");
    });
    // receiver thread 1
    let receiver_handle = spawn(move || {
        for message in rx3 {
            println!("Case 2 Receiver 1 received message: {}", message);
        }
        println!("Case 2 Receiver 1 closed");
    });
    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();
}
