//! Message processing context
//!
//! This object will constructs for each new message received by an actor
use crate::actors::actor_ref::ActorRef;
use crate::actors::actor_system::ActorSystem;
use crate::common::tsafe::TSafe;

pub struct ActorContext {

    /// Who send the current message
    pub sender: ActorRef,

    /// Own actor reference
    pub self_: ActorRef,

    /// Actor system where actor is work
    pub system: TSafe<ActorSystem>
}

impl ActorContext {
    pub fn new(sender: ActorRef, self_: ActorRef, system: TSafe<ActorSystem>) -> ActorContext {
        ActorContext {
            sender,
            self_,
            system
        }
    }
}