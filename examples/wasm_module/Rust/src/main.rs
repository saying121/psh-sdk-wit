mod wit;

use crate::wit::profiling::{
    perf::{
        config::{Config, Cpu, Event, EventScope, ExtraConfig, HardwareEvent, Process},
        counter::Counter,
    },
    system,
};

fn main() {
    println!("{:#?}", system::os::info());
    let config = Config {
        event: Event::Hardware(HardwareEvent::CpuCycles),
        scopes: vec![
            EventScope::User,
            EventScope::Kernel,
            EventScope::Hv,
            EventScope::Idle,
            EventScope::Guest,
            EventScope::Host,
        ],
        extra_config: ExtraConfig {
            pinned: false,
            exclusive: false,
            inherit: false,
            inherit_stat: false,
            inherit_thread: false,
            enable_on_exec: false,
            remove_on_exec: false,
        },
    };
    let counter = Counter::new(Process::Current, Cpu::Any, &config).unwrap();
    counter.enable().unwrap();
    println!("do something here...");
    counter.disable().unwrap();
    let stat = counter.stat().unwrap();
    println!("Counter Stat: {:#?}", stat);
}
