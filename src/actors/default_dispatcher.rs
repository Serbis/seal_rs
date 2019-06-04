//NOTOK
use crate::executors::execution_context::{ExecutionContext, ExecutorTask};
use crate::actors::dispatcher::Dispatcher;
use crate::actors::actor_cell::ActorCell;
use crate::actors::local_actor_ref::LocalActorRef;
use crate::actors::abstract_actor_ref::AbstractActorRef;
use crate::actors::actor_context::ActorContext;
use crate::actors::envelope::Envelope;
use crate::actors::mailbox::Mailbox;
use crate::actors::actor::{Actor, PoisonPill};
use crate::common::tsafe::TSafe;
use std::collections::vec_deque::VecDeque;
use std::sync::{Mutex, Arc, Condvar};
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::any::Any;


pub type AsyncBlockQueue = VecDeque<ExecutorTask>;
//TODO нужно заменить все Mutex это возможно на RwLock
pub struct DefaultDispatcher {
    dv: i32,
    t_count: u32,
    a_blocks: Vec<TSafe<AsyncBlock>>
}

impl DefaultDispatcher {
    pub fn new(t_count: u32) -> DefaultDispatcher {
        DefaultDispatcher {
            dv: -1,
            t_count,
            a_blocks: Vec::new()
        }
    }

    pub fn invoke(mailbox: &TSafe<Mailbox + Send>, actor: &TSafe<Actor + Send>, cell: &TSafe<ActorCell>) {
        let envelope = {
            let mut mailbox = mailbox.lock().unwrap();
            if mailbox.has_messages() {
                Some(mailbox.dequeue())
            } else {
                None
            }
        };

        if envelope.is_some() {
            let envelope = envelope.unwrap();

            let sender: Box<AbstractActorRef + Send> = {
                if envelope.sender.is_some() {
                    envelope.sender.unwrap()
                } else {
                    let mut system = envelope.system.lock().unwrap();
                    let dead_letters = system.dead_letters();
                    dead_letters
                }
            };


            let msg = envelope.message;

            let mut actor = actor.lock().unwrap();
            let ctx = ActorContext::new( sender.clone(), envelope.receiver.clone(), envelope.system.clone());
            let handled = actor.receive(&msg, ctx);

            if !handled {
                let handled2 = DefaultDispatcher::internal_receive(mailbox, &msg, cell);
                if !handled2 {
                    let mut dead_letters = {
                        let mut system = envelope.system.lock().unwrap();
                        let dead_letters = system.dead_letters();
                        dead_letters
                    };
                    dead_letters.cell().lock().unwrap().send(&dead_letters.cell(), msg, Some(sender), envelope.receiver );

                }
            }
        }

        mailbox.lock().unwrap().set_planned(false);
    }

    pub fn internal_receive(mailbox: &TSafe<Mailbox + Send>, msg: &Box<Any + Send>, cell: &TSafe<ActorCell>) -> bool {

        if let Some(PoisonPill {}) = msg.downcast_ref::<PoisonPill>() {
            let mut cell_u = cell.lock().unwrap();
            cell_u.suspend();
            // +++ cell.actor.timers().cancelAll();
            let dead_letters = cell_u.system.lock().unwrap().dead_letters();
            mailbox.lock().unwrap().clean_up(Box::new(LocalActorRef::new(cell.clone(), cell_u.path.clone())), dead_letters);
            cell_u.stop(cell.clone());
        } else {
            return false
        }

        true
    }

    pub fn new_task(f: Box<Fn() -> () + Send>) -> ExecutorTask  {
        tsafe!((f))
    }

    fn find_lowest_block(self: &mut Self) -> usize {
        let mut min = 1000000000;
        let mut min_q = 0;
        let mut qn = 0;

        for q in self.a_blocks.iter() {
            let len = q.lock().unwrap().queue.len();
            if len < min {
                min = len;
                min_q = qn;
            }
            qn = qn + 1;
        }

        min_q
    }

    pub fn sdv(self: &mut Self, dv: i32) {
        self.dv = dv;
        println!("{}", self.dv);
    }
}

impl ExecutionContext for DefaultDispatcher {
    fn register_for_execution(self: &mut Self, bid: i32, f: ExecutorTask) {
        //println!("register_for_execution - 0");
        if bid >= 0 {
            let bid = bid as usize;
            let x = &mut self.a_blocks[bid].lock().unwrap();
            x.add_task(f);
        } else {
            let lowest = self.find_lowest_block();
            let x = &mut self.a_blocks[lowest].lock().unwrap();
            x.add_task(f);
        }

        //println!("register_for_execution - 1");

    }


    fn run(self: &mut Self) {
        for n in 0..self.t_count {
            let mut ab = AsyncBlock::new();
            self.a_blocks.push(ab)
        }
    }
}

impl Dispatcher for DefaultDispatcher {

    fn dispatch(self: &mut Self, cell: TSafe<ActorCell>, bid: u32, mailbox: TSafe<Mailbox + Send>, actor: TSafe<Actor + Send>, envelope: Envelope) {
        let mut mailbox_u = mailbox.lock().unwrap();
        mailbox_u.enqueue(envelope);
        //if !mailbox_u.is_planned() {
            //mailbox_u.set_planned(true);

            let mailbox = mailbox.clone();
            let f = DefaultDispatcher::new_task(Box::new(move || {
                DefaultDispatcher::invoke(&mailbox, &actor, &cell)
            }));

            self.register_for_execution(bid as i32, f)
        //}
    }

    fn obtain_bid(self: &mut Self) -> u32 {
        self.find_lowest_block() as u32
    }
}

// --------------- INTERNAL API ---------------




struct AsyncBlock {
    cvar: Arc<Condvar>,
    cond_m: Arc<Mutex<bool>>,
    queue: AsyncBlockQueue
}

impl AsyncBlock {
    pub fn new() -> TSafe<AsyncBlock> {
        let ab = AsyncBlock {
            cvar: Arc::new(Condvar::new()),
            cond_m: Arc::new(Mutex::new(false)),
            queue: VecDeque::new()
        };

        let ab_arc: TSafe<AsyncBlock> = tsafe!(ab);
        let ab_arc_clone = ab_arc.clone();


        thread::spawn(move || {
            let cvar = {
                let ab =  &mut ab_arc_clone.lock().unwrap();
                ab.cvar.clone()
            };

            let cond_m = {
                let ab =  &mut ab_arc_clone.lock().unwrap();
                ab.cond_m.clone()
            };

            loop {
                //let mut wait = false;

                let f = {
                    let ab = &mut ab_arc_clone.lock().unwrap();

                    let q = &mut ab.queue;
                    if q.len() > 0 {
                        let task = q.pop_front().unwrap();
                        Some(task.clone())
                    } else {
                        None
                        //wait = true;
                    }
                };


                if f.is_none() {
                    let m = cond_m.lock().unwrap();
                    cvar.wait(m);
                } else {
                    f.unwrap().lock().unwrap()();
                }
            }
        });

        ab_arc
    }

    pub fn add_task(self: &mut Self, f: ExecutorTask) {
        self.queue.push_back(f);
        self.cvar.notify_one();
    }
}

