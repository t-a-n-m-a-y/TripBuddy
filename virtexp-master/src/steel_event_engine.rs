use crate::event_engine::DomainAction::{Noop, Suspend};
use crate::event_engine::DomainEvent::{ComputeTimeExceeded, MemoryLimitExceeded};
use crate::event_engine::{DomainAction, DomainEvent, EventEngine, EventGenerator, HandlingError};
use steel::primitives::lists::{cons, steel_pair};
use steel::rerrs::ErrorKind;
use steel::rvals::{FromSteelVal, IntoSteelVal, SteelString};
use steel::steel_vm::engine::Engine;
use steel::{list, SteelErr, SteelVal};
use virt::domain::{Domain, DomainInfo};

impl FromSteelVal for DomainAction {
    fn from_steelval(val: &SteelVal) -> steel::rvals::Result<Self> {
        match val {
            SteelVal::Closure(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::BoolV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::NumV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::IntV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::Rational(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::CharV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::VectorV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::Void => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::StringV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::FuncV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::SymbolV(symbol) => {
                if symbol.eq(&SteelString::from("suspend")) {
                    Ok(Suspend)
                } else if symbol.eq(&SteelString::from("noop")) {
                    Ok(Noop)
                } else {
                    Err(SteelErr::new(
                        ErrorKind::ConversionError,
                        "Not a domain action".parse()?,
                    ))
                }
            }
            SteelVal::Custom(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::HashMapV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::HashSetV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::CustomStruct(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::PortV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::IterV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::ReducerV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::FutureFunc(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::FutureV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::StreamV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::BoxedFunction(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::ContinuationFunction(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::ListV(list) => {
                if list.len() == 2 {
                    let name = &list[0];
                    let migrate = SteelVal::SymbolV(SteelString::from("migrate"));
                    let uri = &list[1];
                    let uri = uri.as_symbol().ok_or(SteelErr::new(
                        ErrorKind::ConversionError,
                        "Unable to parse symbol".parse()?,
                    ))?;
                    if name.eq(&migrate) {
                        Ok(DomainAction::Migrate(uri.to_string()))
                    } else {
                        Err(SteelErr::new(
                            ErrorKind::ConversionError,
                            "Invalid action name".parse()?,
                        ))
                    }
                } else {
                    Err(SteelErr::new(
                        ErrorKind::ConversionError,
                        "List should be of length 2".parse()?,
                    ))
                }
            }
            SteelVal::Pair(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::MutFunc(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::BuiltIn(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::MutableVector(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::BoxedIterator(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::SyntaxObject(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::Boxed(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::HeapAllocated(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::Reference(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::BigNum(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::BigRational(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::Complex(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
            SteelVal::ByteVector(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not a domain action".parse()?,
            )),
        }
    }
}
fn extract_action_from_proc_name(
    engine: &mut Engine,
    name: &str,
    args: Vec<SteelVal>,
) -> Result<DomainAction, HandlingError> {
    match engine.call_function_by_name_with_args(name, args) {
        Ok(val) => {
            if let Ok(action) = DomainAction::from_steelval(&val) {
                Ok(action)
            } else {
                Err(HandlingError::EventHandlingError)
            }
        }
        Err(_) => Err(HandlingError::EventHandlingError),
    }
}

impl EventEngine<DomainEvent, DomainAction, HandlingError> for Engine {
    fn handle(&mut self, event: DomainEvent) -> Result<DomainAction, HandlingError> {
        match event {
            DomainEvent::MemoryLimitExceeded(memory) => extract_action_from_proc_name(
                self,
                "memory-limit-exceeded",
                vec![SteelVal::from(memory)],
            ),
            DomainEvent::ComputeTimeExceeded(time) => extract_action_from_proc_name(
                self,
                "compute-time-exceeded",
                vec![SteelVal::from(time)],
            ),
        }
    }
}

impl FromSteelVal for DomainEvent {
    fn from_steelval(val: &SteelVal) -> steel::rvals::Result<Self> {
        match val {
            SteelVal::Closure(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::BoolV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::NumV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::IntV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::Rational(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::CharV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::VectorV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::Void => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::StringV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::FuncV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::SymbolV(sym) => {
                if sym.eq(&SteelString::from("memory-limit-exceeded")) {
                    Ok(MemoryLimitExceeded(0u64))
                } else if sym.eq(&SteelString::from("compute-time-exceeded")) {
                    Ok(ComputeTimeExceeded(0u64))
                } else {
                    Err(SteelErr::new(ErrorKind::ConversionError, "Not an event".parse()?))
                }
            },
            SteelVal::Custom(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::HashMapV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::HashSetV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::CustomStruct(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::PortV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::IterV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::ReducerV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::FutureFunc(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::FutureV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::StreamV(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::BoxedFunction(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::ContinuationFunction(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::ListV(list) => {
                if list.len() == 2 {
                    let compute_event_symbol = (SteelString::from("compute-time-exceeded"));
                    let memory_event_symbol = (SteelString::from("memory-limit-exceeded"));
                    let first = &list[0];
                    let Some(first) = first.as_symbol() else {
                        panic!("Not a string!!!")
                    };
                    let second = &list[1];
                    let second = second.as_usize().ok_or(SteelErr::new(
                        ErrorKind::ConversionError,
                        "Unable to parse".parse()?,
                    ))?;
                    if first.eq(&compute_event_symbol) {
                        Ok(ComputeTimeExceeded(second as u64))
                    } else if first.eq(&memory_event_symbol) {
                        Ok(MemoryLimitExceeded(second as u64))
                    } else {
                        Err(SteelErr::new(
                            ErrorKind::ConversionError,
                            "Not an event".parse()?,
                        ))
                    }
                } else {
                    Err(SteelErr::new(
                        ErrorKind::ConversionError,
                        "Not an event".parse()?,
                    ))
                }
            }
            SteelVal::Pair(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::MutFunc(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::BuiltIn(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::MutableVector(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::BoxedIterator(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::SyntaxObject(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::Boxed(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::HeapAllocated(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::Reference(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::BigNum(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::BigRational(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::Complex(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
            SteelVal::ByteVector(_) => Err(SteelErr::new(
                ErrorKind::ConversionError,
                "Not an event".parse()?,
            )),
        }
    }
}

fn domain_info_to_steel_val(domain_info: DomainInfo) -> SteelVal {
    return list!(
        domain_info.memory,
        domain_info.max_mem,
        domain_info.cpu_time,
        domain_info.nr_virt_cpu
    );
}

impl EventGenerator<Domain, DomainEvent> for Engine {
    fn generate(&mut self, dom: &Domain) -> Vec<DomainEvent> {
        println!("Monitoring domain {:?}", dom.get_name());
        let Ok(info) = dom.get_info() else {
            panic!("Unable to extract domain information")
        };
        let mut answer = Vec::new();
        let results = self
            .call_function_by_name_with_args("generate", vec![domain_info_to_steel_val(info)])
            .expect("Unable to execute 'generate' procedure");
        return if let Some(list) = results.list() {
            for action in list {
                let action = DomainEvent::from_steelval(action);
                if let Ok(action) = action {
                    answer.push(action);
                }
            }
            answer
        } else {
            let action = DomainEvent::from_steelval(&results).expect("Expected event");
            vec![action]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::event_engine::{run_event_loop, DomainEvent, EventEngine};
    use std::time::Duration;
    use steel::steel_vm::engine::Engine;
    use virt::connect::Connect;
    use virt::sys::{VIR_CONNECT_LIST_DOMAINS_ACTIVE, VIR_CONNECT_LIST_DOMAINS_INACTIVE};

    #[test]
    fn steel_event_engine_test() {
        let mut engine = Engine::new();
        engine
            .run(
                r#"
        (define (memory-limit-exceeded mem)
            (begin
                (displayln "In memory limit event handler")
                'noop))
        (define (compute-time-exceeded time)
            (begin
                (displayln "In compute time event handler")
                'noop))
        "#,
            )
            .expect("Failed to run Steel code");
        engine
            .handle(DomainEvent::ComputeTimeExceeded(5))
            .expect("Failed to run compute time handler");
        engine
            .handle(DomainEvent::MemoryLimitExceeded(40))
            .expect("Failed to run memory limit handler");
    }

    #[test]
    fn steel_event_main_loop() {
        let mut engine = Engine::new();
        engine
            .run(
                r#"
        (define (memory-limit-exceeded mem)
            (begin
                (displayln "In memory limit event handler")
                'noop))
        (define (compute-time-exceeded time)
            (begin
                (displayln "In compute time event handler")
                'noop))
        "#,
            )
            .expect("Failed to run Steel code");
        let uri = "qemu:///system";
        let conn = Connect::open(uri).expect("Unable to connect to hypervisor");
        let domains = conn
            .list_all_domains(VIR_CONNECT_LIST_DOMAINS_ACTIVE | VIR_CONNECT_LIST_DOMAINS_INACTIVE)
            .expect("Unable to extract domain");
        let first = &domains[0];
        run_event_loop(
            first,
            Duration::from_millis(1000),
            |event| engine.handle(event),
            |domain| {
                vec![
                    DomainEvent::ComputeTimeExceeded(50),
                    DomainEvent::MemoryLimitExceeded(400),
                ]
            },
        );
    }
    #[test]
    fn steel_shutdown() {
        let mut engine = Engine::new();
        engine
            .run(
                r#"
        (define (memory-limit-exceeded mem)
            (begin
                (displayln "In memory limit event handler")
                'noop))
        (define (compute-time-exceeded time)
            'suspend)
        "#,
            )
            .expect("Failed to run Steel code");
        let uri = "qemu:///system";
        let conn = Connect::open(uri).expect("Unable to connect to hypervisor");
        let domains = conn
            .list_all_domains(VIR_CONNECT_LIST_DOMAINS_ACTIVE | VIR_CONNECT_LIST_DOMAINS_INACTIVE)
            .expect("Unable to extract domain");
        let first = &domains[0];
        run_event_loop(
            first,
            Duration::from_millis(1000),
            |event| engine.handle(event),
            |domain| {
                vec![
                    DomainEvent::ComputeTimeExceeded(50),
                    DomainEvent::MemoryLimitExceeded(400),
                ]
            },
        );
    }
    #[test]
    fn steel_migrate() {
        let mut engine = Engine::new();
        engine
            .run(
                r#"
        (define (memory-limit-exceeded mem)
            (begin
                (displayln "In memory limit event handler")
                'noop))
        (define (compute-time-exceeded time)
            (begin
                (displayln "In memory limit event handler")
                '(migrate qemu:///shared)))
        "#,
            )
            .expect("Failed to run Steel code");
        let uri = "qemu:///system";
        let conn = Connect::open(uri).expect("Unable to connect to hypervisor");
        let domains = conn
            .list_all_domains(VIR_CONNECT_LIST_DOMAINS_ACTIVE | VIR_CONNECT_LIST_DOMAINS_INACTIVE)
            .expect("Unable to extract domain");
        let first = &domains[0];
        run_event_loop(
            first,
            Duration::from_millis(1000),
            |event| engine.handle(event),
            |domain| {
                vec![
                    DomainEvent::ComputeTimeExceeded(50),
                    DomainEvent::MemoryLimitExceeded(400),
                ]
            },
        );
    }
}
