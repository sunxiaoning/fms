use crate::sm::msg::Message;
use crate::sm::state::State;
use std::fmt::Debug;
pub struct StateContext<'a, S: PartialEq + Debug + Clone, E: PartialEq> {
    message: &'a Message<E>,
    tran: &'a Transition<S, E>,
}

impl<'a, S: PartialEq + Debug + Clone, E: PartialEq> StateContext<'a, S, E> {
    pub fn new(tran: &'a Transition<S, E>, message: &'a Message<E>) -> StateContext<'a, S, E> {
        StateContext { tran, message }
    }

    pub fn tran(&self) -> &Transition<S, E> {
        self.tran
    }

    pub fn message(&self) -> &Message<E> {
        self.message
    }
}

pub type Action<S, E> = fn(ctx: &StateContext<S, E>) -> Result<(), &'static str>;

pub type Guard<S, E> = fn(ctx: &StateContext<S, E>) -> bool;

pub struct Transition<S: PartialEq + Debug + Clone, E: PartialEq> {
    source: State<S>,
    target: State<S>,
    event: E,
    guard: Option<Guard<S, E>>,
    action: Option<Action<S, E>>,
}

impl<S: PartialEq + Debug + Clone, E: PartialEq> Transition<S, E> {
    pub fn transit(&self, ctx: &StateContext<S, E>) -> Result<bool, &str> {
        if let Some(gurd) = self.guard {
            Ok(gurd(ctx))
        } else {
            Ok(true)
        }
    }

    pub fn new(
        source: State<S>,
        target: State<S>,
        event: E,
        action: Option<Action<S, E>>,
        guard: Option<Guard<S, E>>,
    ) -> Self {
        Transition {
            source,
            target,
            event,
            guard,
            action,
        }
    }

    pub fn source(&self) -> &State<S> {
        &self.source
    }

    pub fn target(&self) -> &State<S> {
        &self.target
    }

    pub fn event(&self) -> &E {
        &self.event
    }

    pub fn action<'a>(&self) -> Option<fn(&'a StateContext<S, E>) -> Result<(), &'a str>> {
        self.action
    }
}

pub struct TransitionBuilder<S: PartialEq + Debug + Clone, E: PartialEq> {
    source: Option<State<S>>,
    target: Option<State<S>>,
    event: Option<E>,
    action: Option<Action<S, E>>,
    guard: Option<Guard<S, E>>,
}

impl<S: PartialEq + Debug + Clone, E: PartialEq> TransitionBuilder<S, E> {
    pub fn build(&mut self) -> Transition<S, E> {
        Transition::new(
            self.source.take().expect("source state absent!"),
            self.target.take().expect("target state absent!"),
            self.event.take().expect("event absent"),
            self.action.take(),
            self.guard.take(),
        )
    }
    pub fn guard(&mut self, guard: Option<Guard<S, E>>) -> &mut Self {
        self.source.as_ref().expect("source absent");
        self.target.as_ref().expect("target absent");
        self.event.as_ref().expect("event absent");
        self.guard = guard;
        self
    }
    pub fn action(&mut self, act: Option<Action<S, E>>) -> &mut Self {
        self.source.as_ref().expect("source absent");
        self.target.as_ref().expect("target absent");
        self.event.as_ref().expect("event absent");
        self.action = act;
        self
    }
    pub fn event(&mut self, event: E) -> &mut Self {
        self.source.as_ref().expect("source absent");
        self.target.as_ref().expect("target absent");
        self.event = Some(event);
        self
    }
    pub fn target(&mut self, target: State<S>) -> &mut Self {
        self.source.as_ref().expect("source absent");
        self.target = Some(target);
        self
    }
    pub fn source(&mut self, source: State<S>) -> &mut Self {
        self.source = Some(source);
        self
    }
    pub fn new() -> Self {
        TransitionBuilder {
            source: None,
            target: None,
            event: None,
            action: None,
            guard: None,
        }
    }
}
