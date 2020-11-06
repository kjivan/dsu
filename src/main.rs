use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

fn main() {
    let mut team = ["Sitora", "Gerolamo", "Salena", "Meine"];
    let start_wait = time::Duration::from_secs(3);
    let time_per_person_seconds = 5;
    let warn_at_seconds = 2;

    let mut rng = thread_rng();
    team.shuffle(&mut rng);

    println!("\nDaily Stand-Up Order\n");

    for person in team.iter() {
        println!("{}", person);
    }
    println!();

    thread::sleep(start_wait);

    let stdin_channel = spawn_stdin_channel();
    for person in team.iter() {
        println!("Go for it {}!", person);

        for second in 0..time_per_person_seconds {
            if second == warn_at_seconds {
                println!("Only {} seconds left!", warn_at_seconds);
            }
            thread::sleep(time::Duration::from_secs(1));
            match stdin_channel.try_recv() {
                Ok(_) => break,
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
