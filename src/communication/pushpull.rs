use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver};
use communication::Observer;

pub trait Pushable<T> : 'static { fn push(&mut self, data: T); }    // like observer
pub trait Pullable<T> : 'static { fn pull(&mut self) -> Option<T>; } // like iterator

impl<T:'static> Pushable<T> for Rc<RefCell<Vec<T>>> { fn push(&mut self, data: T) { self.borrow_mut().push(data); } }
impl<T:'static> Pullable<T> for Rc<RefCell<Vec<T>>> { fn pull(&mut self) -> Option<T> { self.borrow_mut().pop() } }

impl<T:Send> Pushable<T> for Sender<T> { fn push(&mut self, data: T) { self.send(data); } }
impl<T:Send> Pullable<T> for Receiver<T> { fn pull(&mut self) -> Option<T> { self.try_recv().ok() }}

impl<T:Send> Pushable<T> for Box<Pushable<T>> { fn push(&mut self, data: T) { (**self).push(data); } }
impl<T:Send> Pullable<T> for Box<Pullable<T>> { fn pull(&mut self) -> Option<T> { (**self).pull() } }

pub struct PushableObserver<T:Send, D:Send+Clone, P: Pushable<(T, Vec<D>)>> {
    pub data:       Vec<D>,
    pub pushable:   P,
}

impl<T:Send+Clone, D:Send+Clone, P: Pushable<(T, Vec<D>)>> Observer for PushableObserver<T,D,P> {
    type Time = T;
    type Data = D;
    #[inline(always)] fn open(&mut self,_time: &T) { }
    #[inline(always)] fn push(&mut self, data: &D) { self.data.push(data.clone()); }
    #[inline(always)] fn shut(&mut self, time: &T) { let vec = mem::replace(&mut self.data, Vec::new()); self.pushable.push((time.clone(), vec)); }
}
