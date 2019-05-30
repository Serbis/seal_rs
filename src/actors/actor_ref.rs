//! Reference to instance of some actor. It is an interface for interacting with the internal
//! logic of the actor cell.

use crate::common::tsafe::TSafe;
use crate::actors::actor_cell::ActorCell;
use crate::actors::actor_path::ActorPath;
use std::fmt;
use std::any::Any;

pub struct ActorRef {
    pub cell: TSafe<ActorCell>,
    pub path: TSafe<ActorPath>
}

impl ActorRef {

   /// Creates a new reference. This method should never be invoked by application code. This
   /// constructor is used by internal API. Direct use from the user code is prohibited.
    pub fn new(cell: TSafe<ActorCell>, path: TSafe<ActorPath>) -> ActorRef {
        ActorRef {
            cell,
            path
        }
    }

    /// Send message to the actor behind the reference. In the first argument passed message for
    /// send, and in the second argument specified sender reference. This reference will be injected
    /// to ctx.sender field the actor context object. If sender was does not specified, ctx.sender
    /// will be filled with the deadLetter actor reference. Setting up None as sender reference is
    /// useful in case, when tell operation is
    /// called from outside of the actor system.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn tell(self: &mut Self, msg: Box<Any + Send + 'static>, rself: Option<ActorRef>) {
        self.cell.lock().unwrap().send(&self.cell, msg, rself, ActorRef::new(self.cell.clone(), self.path.clone()))
    }

    /// Return copy of the actor path object
    pub fn path(self: &mut Self) -> ActorPath {
        self.path.lock().unwrap().clone()
    }
}

impl Clone for ActorRef {
    fn clone(&self) -> Self {
        ActorRef {
            cell: self.cell.clone(),
            path: self.path.clone()
        }
    }
}

impl PartialEq for ActorRef {
    fn eq(&self, other: &Self) -> bool {
        true
        //self.path == other.path
    }
}

impl Eq for ActorRef {}

impl fmt::Display for ActorRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorRef ({})", self.path.lock().unwrap())
    }
}