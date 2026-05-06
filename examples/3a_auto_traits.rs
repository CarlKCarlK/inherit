use std::rc::Rc;
use std::thread;

fn spawn_if_send<T: Send + 'static>(value: T) {
    let handle = thread::spawn(move || {
        let _ = value;
        println!("moved to thread");
    });

    handle.join().unwrap();
}

fn main() {
    spawn_if_send(String::from("owned"));

    let _not_send = Rc::new(5);
    println!("Rc<T> is not Send, so it cannot be passed to spawn_if_send");
}
