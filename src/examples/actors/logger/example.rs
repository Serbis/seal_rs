use crate::actors::default_dispatcher::DefaultDispatcher;
use crate::actors::dispatcher::Dispatcher;
use crate::actors::actor_cell::ActorCell;
use crate::actors::actor_ref::ActorRef;
use crate::actors::props::Props;
use crate::actors::unbound_mailbox::UnboundMailbox;
use crate::actors::actor_path::ActorPath;
use crate::actors::actor_system::ActorSystem;
use crate::actors::envelope::Envelope;
use crate::examples::actors::logger::logger;
use crate::examples::actors::logger::stdout_writer;
use crate::examples::actors::logger::file_writer;
use crate::common::tsafe::TSafe;
use crate::executors::execution_context::ExecutionContext;
use std::thread;
use std::sync::{Mutex, Arc};
use std::rc::Rc;
use std::time::Duration;

pub fn run() {
    let mut system = ActorSystem::new();
    system.lock().unwrap().run();

    let mut logger =  {
        let mut system = system.lock().unwrap();
        let file_writer = system.actor_of(file_writer::props("/tmp/log"), Some("file_writer"));
        let stdout_writer = system.actor_of(stdout_writer::props(), Some("stdout_writer"));
        system.actor_of(logger::props(file_writer, stdout_writer), Some("logger"))
    };

    logger.tell(Box::new(logger::Log { text: String::from("To file log"), target: logger::LogTarget::File }), None);
    logger.tell(Box::new(logger::Log { text: String::from("To stdout log"), target: logger::LogTarget::StdOut }), None);
}