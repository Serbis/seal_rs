use crate::actors::default_dispatcher::DefaultDispatcher;
use crate::actors::dispatcher::Dispatcher;
use crate::actors::actor_cell::ActorCell;
use crate::actors::actor_ref::ActorRef;
use crate::actors::props::Props;
use crate::actors::unbound_mailbox::UnboundMailbox;
use crate::actors::actor_path::ActorPath;
use crate::actors::actor_system::ActorSystem;
use crate::actors::envelope::Envelope;
use crate::examples::actors::basic::basic_actor;

use crate::common::tsafe::TSafe;
use crate::executors::execution_context::ExecutionContext;
use std::thread;
use std::sync::{Mutex, Arc};
use std::rc::Rc;
use std::time::Duration;

pub fn run() {
    let mut system = ActorSystem::new();
    system.lock().unwrap().run();

    let mut printer = system.lock().unwrap()
        .actor_of(basic_actor::props(), Some("printer"));

    let msg = Box::new(basic_actor::Print { text: String::from("Hello world!") });
    printer.tell(msg, None);
}