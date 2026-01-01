mod ffi;
mod poll;

use poll::Poll;

fn main() -> Result<(), std::io::Error> {
    let new_poll = Poll::new();
    let registry = new_poll.registry();
    println!("Hello, world!{:?}", registry);
    Ok(())
}
