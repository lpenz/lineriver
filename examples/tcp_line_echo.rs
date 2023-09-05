use lineriver::LineRead;
use polling::{Event, Events, Poller};
use std::collections::HashMap;
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create non-blocking TcpListener:
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    listener.set_nonblocking(true)?;
    let listener_key = 0;
    // Create poller, register interest in listener:
    let poller = Poller::new()?;
    unsafe {
        poller.add(&listener, Event::readable(listener_key))?;
    };
    let mut events = Events::new();
    let mut clients = HashMap::new();
    let mut next_key = listener_key + 1;
    loop {
        events.clear();
        poller.wait(&mut events, None)?;
        for ev in events.iter() {
            if ev.key == listener_key {
                // Perform a non-blocking accept operation.
                let (client, addr) = listener.accept()?;
                eprintln!("{}: connected", addr);
                // Add client to list.
                let client_reader = lineriver::LineReader::new(client)?;
                unsafe {
                    poller.add(&client_reader, Event::readable(next_key))?;
                };
                clients.insert(next_key, (addr, client_reader));
                next_key += 1;
                // Set interest in the next readability event from listener.
                poller.modify(&listener, Event::readable(listener_key))?;
            } else {
                // Event if from a client
                let (addr, reader) = clients.get_mut(&ev.key).expect("client not found");
                if !reader.eof() {
                    reader.read_available()?;
                    for line in reader.lines_get() {
                        print!("{}: {}", addr, line);
                    }
                    // Set interest in the next readability event from client.
                    poller.modify(&clients[&ev.key].1, Event::readable(ev.key))?;
                } else {
                    // eof, remove the client
                    print!("{}: eof", addr);
                    clients.remove(&ev.key);
                }
            }
        }
    }
}
