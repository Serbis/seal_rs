use crate::actors::local_actor_ref::LocalActorRef;
use crate::actors::actor_ref_factory::ActorRefFactory;
use crate::actors::abstract_actor_ref::AbstractActorRef;
use std::any::Any;

//TODO docs
pub trait AbstractActorSystem: ActorRefFactory {

}

//TODO остановка акторной системы
//TODO вызов PostStop из drop и проверка теории закольцованных ссылко и ручного сброса актора (drop)
