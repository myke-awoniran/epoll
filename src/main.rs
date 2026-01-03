mod ffi;
mod poll;

use ffi::Event;
use poll::Poll;
use std::collections::HashSet;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::TcpStream;

fn get_req(path: &str) -> Vec<u8> {
    format!(
        "GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\
        \r\n"
    )
    .into()
}

fn handle_events(
    events: &[Event],
    streams: &mut [TcpStream],
    handled_ids: &mut HashSet<usize>,
) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];
        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    if !handled_ids.insert(index) {
                        break;
                    }

                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n------\n");
                }
                // Not ready to read in a non-blocking manner. This could
                // happen even if the event was reported as ready
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
    }
    Ok(handled_events)
}

fn main() -> Result<()> {
    let mut poll = Poll::new()?;
    let num_events = 10;
    let mut streams = vec![];
    let server_addr = "127.0.0.1:8080";

    for i in 0..num_events {
        let delay = (num_events - 1) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = TcpStream::connect(&server_addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(&request)?;
        println!("REQUEST SENT: {:?}", request);
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        println!("REGISTERED: {:?}", stream);
        streams.push(stream);
    }

    let mut handled_ids = HashSet::new();

    let mut handled_events = 0;
    while handled_events < num_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }

    println!("FINISHED");

    // let registry = new_poll.registry();
    // println!("Hello, world!{:?}", registry);
    // series_of_bitmask();
    check_bitmask(series_of_bitmask());

    Ok(())
}

fn series_of_bitmask() -> i32 {
    let bitmask_a: i32 = 1 << 31;
    let bitmask_b: i32 = 0xb1;
    let bitmask: i32 = bitmask_a | bitmask_b;
    println!("{bitmask_a:032b}");
    println!("{bitmask_b:032b}");
    println!("{bitmask:032b}");
    bitmask
}

fn check_bitmask(bitmask: i32) {
    const EPOLLIN: i32 = 0x1;
    const EPOLLET: i32 = 1 << 31;
    const EPOLLONESHOT: i32 = 0x40000000;
    let read = bitmask & EPOLLIN != 0;
    let et = bitmask & EPOLLET != 0;
    let oneshot = bitmask & EPOLLONESHOT != 0;
    println!("read_event? {read}, edge_triggered: {et}, oneshot?: {oneshot}")
}
