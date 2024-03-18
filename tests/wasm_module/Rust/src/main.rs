mod bindings;

use crate::bindings::profiling::{
    perf::{
        config::{Config, Cpu, Event, EventScope, ExtraConfig, HardwareEvent, Process},
        counter::Counter,
    },
    system,
};

extern crate alloc;

fn main() {
    println!("{:?}", system::os::get_distro_version());
    println!("{:?}", system::os::get_kernel_version());
    let config = Config {
        event: Event::Hardware(HardwareEvent::CpuCycles),
        scopes: vec![EventScope::User],
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
