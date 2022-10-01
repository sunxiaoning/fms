use super::{msg::EventId, state::StateId, trans::Transition};

pub trait StateMachineListener<S: StateId, E: EventId> {
    fn transion(&self, _: &Transition<S, E>) {}
}

pub struct StateMachineNotify<S: StateId, E: EventId> {
    listeners: Vec<Box<dyn StateMachineListener<S, E>>>,
}

impl<S: StateId, E: EventId> StateMachineNotify<S, E> {
    pub fn addListener(&mut self, listener: Box<dyn StateMachineListener<S, E>>) {
        self.listeners.push(listener);
    }

    pub fn notify_transition(&self, tran: &Transition<S, E>) {
        for listener in self.listeners.iter() {
            listener.transion(tran);
        }
    }

    pub fn new() -> StateMachineNotify<S, E> {
        StateMachineNotify { listeners: vec![] }
    }
}
