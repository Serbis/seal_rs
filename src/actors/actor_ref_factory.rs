use crate::actors::props::Props;
use crate::actors::abstract_actor_ref::ActorRef;

//TODO docs
pub trait ActorRefFactory {
    fn actor_of(self: &mut Self, props: Props, name: Option<&str>) -> ActorRef;
    fn stop(self: &mut Self, aref: &mut ActorRef);
    fn dead_letters(self: &mut Self) -> ActorRef;
}