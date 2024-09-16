use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use steel::rerrs::ErrorKind;
use steel::rvals::SerializableSteelVal::SymbolV;
use steel::rvals::{FromSteelVal, SteelString};
use steel::{SteelErr, SteelVal};

use crate::event_engine::DomainAction::{Migrate, Noop, Suspend};
use virt::domain::Domain;
use virt::sys;

pub trait EventEngine<Event, Action, HandlingError> {
    fn handle(&mut self, event: Event) -> Result<Action, HandlingError>;
}

pub trait EventGenerator<DomainType, Event> {
    fn generate(&mut self, dom: &DomainType) -> Vec<Event>;
}

pub enum DomainEvent {
    MemoryLimitExceeded(u64),
    ComputeTimeExceeded(u64),
}
pub enum DomainAction {
    Suspend,
    Noop,
    Migrate(String),
}

#[derive(Debug)]
pub enum HandlingError {
    DomainConnectionError,
    EventHandlingError,
}

pub fn run_event_loop<F, G>(domain: &Domain, wait: Duration, mut engine: F, mut generator: G)
where
    F: FnMut(DomainEvent) -> Result<DomainAction, HandlingError>,
    G: FnMut(&Domain) -> Vec<DomainEvent>,
{
    if let Ok(active) = domain.is_active() {
        if active {
            let events = generator(domain);
            for event in events {
                let action = engine(event).expect("Error while handling event");
                match action {
                    Suspend => {
                        // shutdown the VM
                        domain.suspend().expect("Unable to suspend VM");
                        println!(
                            "Domain shutdown {:?} {:?}",
                            domain.get_name(),
                            domain.get_info()
                        );
                    }
                    Noop => {} // do nothing
                    Migrate(uri) => {
                        let Ok(connect) = domain.get_connect() else {
                            panic!("Could not get connection from domain")
                        };
                        println!("Migrating to {}", uri);
                        println!("Starting migration of {:?}", domain.get_name());
                        let mut command = Command::new("virsh");
                        command.arg("migrate");
                        command.arg("--live");
                        command.arg("--unsafe");
                        command.arg("--copy-storage-all");
                        command.arg("--persistent");
                        command.arg(domain.get_name().expect("Failed to get domain name"));
                        command.arg(uri);
                        command.spawn().expect("Unable to trigger migration");
                        let output = command.output().expect("Failed to get output from migration process");
                        println!("{:?}", output);
                    }
                }
            }
        } else {
            return;
        }
    } else {
        return;
    }
    sleep(wait);
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use virt::connect::Connect;
    use virt::sys::{VIR_CONNECT_LIST_DOMAINS_ACTIVE, VIR_CONNECT_LIST_DOMAINS_INACTIVE};

    use crate::event_engine::DomainAction::Noop;
    use crate::event_engine::{run_event_loop, DomainEvent};

    #[test]
    fn dummy_engine() {
        let uri = "qemu:///system";
        let conn = Connect::open(uri).expect("Unable to connect to hypervisor");
        let domains = conn
            .list_all_domains(VIR_CONNECT_LIST_DOMAINS_ACTIVE | VIR_CONNECT_LIST_DOMAINS_INACTIVE)
            .expect("Unable to extract domain");
        let first = &domains[0];
        let cpu_time_limit = 0;
        let memory_limit = 0;
        run_event_loop(
            first,
            Duration::from_micros(0),
            |event| match event {
                DomainEvent::MemoryLimitExceeded(_) => {
                    println!("Memory limit exceeded reported");
                    Ok(Noop)
                }
                DomainEvent::ComputeTimeExceeded(_) => {
                    println!("Compute time exceeded reported");
                    Ok(Noop)
                }
            },
            |domain| {
                let Ok(info) = domain.get_info() else {
                    panic!("Unable to access domain information")
                };
                let mut answer = Vec::new();
                if info.cpu_time > cpu_time_limit {
                    answer.push(DomainEvent::ComputeTimeExceeded(info.cpu_time));
                }
                if info.memory > memory_limit {
                    answer.push(DomainEvent::MemoryLimitExceeded(info.memory));
                }
                return answer;
            },
        );
    }
}
