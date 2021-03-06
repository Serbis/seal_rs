//! Reference to instance of some actor. It is an interface for interacting with the internal
//! logic of the actor cell.

use crate::common::tsafe::TSafe;
use crate::actors::actor_cell::ActorCell;
use crate::actors::abstract_actor_ref::{AbstractActorRef, ActorRef};
use crate::actors::actor_path::ActorPath;
use std::fmt;
use std::any::Any;

pub struct LocalActorRef {
    pub cell: TSafe<ActorCell>,
    pub path: TSafe<ActorPath>
}

impl LocalActorRef {

    /// Creates a new reference. This method should never be invoked by application code. This
    /// constructor is used by internal API. Direct use from the user code is prohibited.
    pub fn new(cell: TSafe<ActorCell>, path: TSafe<ActorPath>) -> LocalActorRef {
        LocalActorRef {
            cell,
            path
        }
    }

    fn inner_clone(self: &Self) -> Box<LocalActorRef> {
        Box::new(LocalActorRef {
            cell: self.cell.clone(),
            path: self.path.clone(),
        })
    }
}

impl AbstractActorRef for LocalActorRef {

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
    fn tell(self: &mut Self, msg: Box<Any + Send + 'static>, rself: Option<Box<AbstractActorRef + Send>>) {
        let cell_cloned = self.cell.clone();
        let path_cloned = self.path.clone();
        let toref = Box::new(LocalActorRef::new(cell_cloned, path_cloned));
        let mut cell = self.cell.lock().unwrap();
        cell.send(&self.cell, msg, rself, toref)    }

    /// Return copy of the actor path object
    fn path(self: &mut Self) -> ActorPath {
        self.path.lock().unwrap().clone()
    }

    fn cell(self: &mut Self) -> TSafe<ActorCell> {
        self.cell.clone()
    }

    fn clone(self: &Self) -> ActorRef {
        self.inner_clone()
    }

    fn as_any(self: &Self) -> Box<Any> {
        Box::new(self.inner_clone())
    }
}

impl fmt::Display for LocalActorRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorRef ({})", self.path.lock().unwrap())
    }
}

/*impl Clone for LocalActorRef {
    fn clone(&self) -> LocalActorRef {
        LocalActorRef {
            cell: self.cell.clone(),
            path: self.path.clone()
        }
    }
}*/

impl PartialEq for LocalActorRef {
    fn eq(&self, _other: &Self) -> bool {
        true
        //self.path == other.path
    }
}

impl Eq for LocalActorRef {}

