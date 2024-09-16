mod domain_event_engine;
mod event_engine;
mod steel_event_engine;

use crate::event_engine::DomainEvent::{ComputeTimeExceeded, MemoryLimitExceeded};
use crate::event_engine::{run_event_loop, EventEngine, EventGenerator};
use std::fs;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Duration;
use steel::steel_vm::engine::Engine;
use virt::connect::Connect;
use virt::sys;

fn main() {
    let uri = std::env::args()
        .nth(1)
        .expect("Hypervisor URI is necessary");
    let script = std::env::args().nth(2).expect("Script source expected");
    let Ok(mut conn) = Connect::open(uri.as_str()) else {
        panic!("Unable to open connection with hypervisor");
    };
    println!("Successfully connected to hypervisor at {}", uri);
    let flags = sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE | sys::VIR_CONNECT_LIST_DOMAINS_INACTIVE;
    let Ok(domains) = conn.list_all_domains(flags) else {
        panic!("Failed to get domains from hypervisor")
    };
    let script = fs::read_to_string(script).expect("Could not read script file");
    let mut engine = Engine::new();
    let mut handler_engine = Engine::new();
    engine
        .run(script.clone())
        .expect("Error while running script");
    handler_engine
        .run(script.clone())
        .expect("Error while running script");
    loop {
        for dom in &domains {
            run_event_loop(
                &dom,
                Duration::from_millis(10),
                |event| engine.handle(event),
                |domain| handler_engine.generate(domain),
            );
        }
    }
    let close = conn.close();
    if close.is_err() {
        eprintln!("Error closing connection to hypervisor");
    }
}
