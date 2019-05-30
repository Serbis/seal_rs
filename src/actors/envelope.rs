//! Internal representation of a message
//!
//! Contains the message itself and elements of context.
use crate::actors::actor_ref::ActorRef;
use crate::actors::actor_system::ActorSystem;
use crate::common::tsafe::TSafe;
use std::any::Any;


pub struct Envelope {

    /// Boxed message
    pub message: Box<Any + Send + 'static>,

    /// Who send this message
    pub sender: Option<ActorRef>,

    /// Who must receive this message
    pub receiver: ActorRef,

    /// Link to the actor system
    pub system: TSafe<ActorSystem>
}

impl Envelope {
    pub fn new(message: Box<Any + Send + 'static>, sender: Option<ActorRef>, receiver: ActorRef, system: TSafe<ActorSystem>) -> Envelope {
        Envelope {
            message,
            sender,
            receiver,
            system
        }
    }
}