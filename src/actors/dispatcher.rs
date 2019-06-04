//NOTOK
use crate::actors::actor_cell::ActorCell;
use crate::actors::envelope::Envelope;
use crate::actors::mailbox::Mailbox;
use crate::actors::actor::Actor;
use crate::common::tsafe::TSafe;
use std::sync::{Arc};

pub trait Dispatcher {
    fn dispatch(self: &mut Self,
                cell: TSafe<ActorCell>,
                bid: u32,
                mailbox: TSafe<Mailbox + Send>,
                actor: TSafe<Actor + Send>,
                envelope: Envelope);

    fn obtain_bid(self: &mut Self) -> u32;
}