use std::fmt::Debug;

struct TransitionBuilder<S: PartialEq + Clone, E: PartialEq> {
    source: Option<State<S>>,
    target: Option<State<S>>,
    event: Option<Event<E>>,
}

impl<S: PartialEq + Clone, E: PartialEq> TransitionBuilder<S, E> {
    fn build(&mut self) -> Transition<S, E> {
        Transition::new(
            self.source.take().expect("source state absent!"),
            self.target.take().expect("target state absent!"),
            self.event.take().expect("event absent"),
        )
    }
    fn event(&mut self, event: Event<E>) -> &mut Self {
        self.event = Some(event);
        self
    }
    fn target(&mut self, target: State<S>) -> &mut Self {
        self.target = Some(target);
        self
    }
    fn source(&mut self, source: State<S>) -> &mut Self {
        self.source = Some(source);
        self
    }
    fn new() -> Self {
        TransitionBuilder {
            source: None,
            target: None,
            event: None,
        }
    }
}

struct Transition<S: PartialEq + Clone, E: PartialEq> {
    source: State<S>,
    target: State<S>,
    event: Event<E>,
    // action: fn() ,
}

impl<S: PartialEq + Clone, E: PartialEq> Transition<S, E> {
    fn transit(&self) -> Result<bool, &str> {
        Ok(true)
    }

    fn new(source: State<S>, target: State<S>, event: Event<E>) -> Self {
        Transition {
            source,
            target,
            event,
        }
    }
}

struct Event<E: PartialEq> {
    id: E,
}

impl<E: PartialEq> PartialEq for Event<E> {
    fn eq(&self, other: &Event<E>) -> bool {
        self.id == other.id
    }
}

impl<E: PartialEq> Event<E> {
    fn build(id: E) -> Self {
        Event { id }
    }
}

#[derive(Debug)]
struct State<S: PartialEq + Clone> {
    id: S,
}

impl<S: PartialEq + Clone> State<S> {
    fn build(id: S) -> Self {
        State { id }
    }
}

impl<S: PartialEq + Clone> PartialEq for State<S> {
    fn eq(&self, other: &State<S>) -> bool {
        self.id == other.id
    }
}

impl<S: PartialEq + Clone> Clone for State<S> {
    fn clone(&self) -> Self {
        State {
            id: self.id.clone(),
        }
    }
}

struct StateMachineBuilder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    init: Option<State<S>>,
    end: Option<State<S>>,
    trans: Option<Vec<Transition<S, E>>>,
    trans_builder: Option<TransitionBuilder<S, E>>,
}

trait SmBuilder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn init(self: Box<Self>, init: State<S>) -> Box<dyn TransStarter<S, E>>;
}



trait TransStarter<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn trans_start(self: Box<Self>) -> Box<dyn Init<S, E>>;
}

trait Init<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn source(self: Box<Self>, source: State<S>) -> Box<dyn Source<S, E>>;
}

trait Source<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn target(self: Box<Self>, target: State<S>) -> Box<dyn Target<S, E>>;
}

trait Target<S: PartialEq + Debug + Debug + Clone, E: PartialEq + Debug> {
    fn event(self: Box<Self>, event: Event<E>) -> Box<StateMachineBuilder<S, E>>;
}

trait TransEnder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn trans_end(self: Box<Self>) -> Box<StateMachineBuilder<S, E>>;
}

impl<S: PartialEq + Debug + Clone, E: PartialEq + Debug> StateMachineBuilder<S, E> {
    fn end(&mut self, end: State<S>) -> &mut StateMachineBuilder<S, E> {
        self.init.as_ref().expect("init absent!");
        assert!(self.trans.as_ref().expect("trans absent!").len() > 0);
        self.end = Some(end);
        self
    }

    fn build(&mut self) -> StateMachine<S, E> {
        StateMachine::new(
            self.init.take().expect("init state absent!"),
            self.end.take().expect("end state absent!"),
            self.trans.take().expect("trans absent!"),
        )
    }

    fn new() -> StateMachineBuilder<S, E> {
        StateMachineBuilder {
            init: None,
            end: None,
            trans: None,
            trans_builder: None,
        }
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> SmBuilder<S, E>
    for StateMachineBuilder<S, E>
{
    fn init(mut self: Box<Self>, init: State<S>) -> Box<dyn TransStarter<S, E>> {
        self.init = Some(init);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> TransStarter<S, E>
    for StateMachineBuilder<S, E>
{
    fn trans_start(mut self: Box<Self>) -> Box<dyn Init<S, E>> {
        self.trans_builder = Some(TransitionBuilder::new());
        self.trans = Some(vec![]);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> Init<S, E>
    for StateMachineBuilder<S, E>
{
    fn source(mut self: Box<Self>, source: State<S>) -> Box<dyn Source<S, E>> {
        let tranb = self.trans_builder.as_mut().expect("trans builder absent");
        tranb.source(source);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> Source<S, E>
    for StateMachineBuilder<S, E>
{
    fn target(mut self: Box<Self>, target: State<S>) -> Box<dyn Target<S, E>> {
        let tranb = self.trans_builder.as_mut().expect("trans builder absent");
        tranb.source.as_ref().expect("source absent");
        tranb.target(target);
        self
    }
}

impl<S: PartialEq + Debug + Clone, E: PartialEq + Debug> Target<S, E>
    for StateMachineBuilder<S, E>
{
    fn event(mut self: Box<Self>, event: Event<E>) -> Box<StateMachineBuilder<S, E>> {
        let tranb = self.trans_builder.as_mut().expect("trans builder absent");
        tranb.source.as_ref().expect("source absent");
        tranb.source.as_ref().expect("target absent");

        tranb.event(event);
        self.trans
            .as_mut()
            .expect("trans absent!")
            .push(tranb.build());
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> TransEnder<S, E>
    for StateMachineBuilder<S, E>
{
    fn trans_end(mut self: Box<Self>) -> Box<StateMachineBuilder<S, E>> {
        self.trans_builder = None;
        self
    }
}

struct StateMachine<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    init: State<S>,
    end: State<S>,
    current: State<S>,
    err: Option<String>,
    trans: Vec<Transition<S, E>>,
}

impl<'a, S: PartialEq + Debug + Clone, E: PartialEq + Debug> StateMachine<S, E> {
    fn is_running(&self) -> bool {
        return !self.current.eq(&self.end);
    }

    fn has_err(&self) -> bool {
        match self.err {
            Some(_) => true,
            None => false,
        }
    }

    fn get_state(&self) -> &State<S> {
        &self.current
    }

    fn send_event(&mut self, e: &Event<E>) -> bool {
        if self.has_err() {
            println!("statemachine err: {}!", self.err.as_ref().unwrap());
            return false;
        }
        if !self.is_running() {
            println!("statemachine not running, event: {:?} not accept!", e.id);
            return false;
        }

        let mut rs = false;
        for trans in self.trans.iter() {
            if trans.source.eq(&self.current) && trans.event.eq(e) {
                let mut err_msg = None;
                rs = trans.transit().unwrap_or_else(|err| {
                    eprintln!(
                        "state machine trans: source: {:?}, event: {:?}, err: {}",
                        trans.source.id, e.id, err
                    );
                    err_msg = Some(String::from(err.clone()));
                    false
                });
                self.err = err_msg;
                if !rs {
                    println!("statemachine event: {:?} not accept!", e.id)
                }
                self.current = trans.target.clone();
                break;
            }
        }
        println!("statemachine event: {:?} not accept!", e.id);
        rs
    }
    fn new(init: State<S>, end: State<S>, trans: Vec<Transition<S, E>>) -> StateMachine<S, E> {
        StateMachine {
            init: init.clone(),
            end: end,
            current: init,
            err: None,
            trans: trans,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug, Clone)]
    enum OrderState {
        I,
        P,
        S,
        F,
    }

    #[derive(PartialEq, Debug)]
    enum OrderEvent {
        Submit,
        Payment,
        Timeout,
    }

    fn init_sm<S, E>() -> StateMachine<OrderState, OrderEvent> {
        Box::new(StateMachineBuilder::new())
            .init(State::build(OrderState::I))
            .trans_start()
            .source(State::build(OrderState::I))
            .target(State::build(OrderState::P))
            .event(Event::build(OrderEvent::Submit))
            .source(State::build(OrderState::P))
            .target(State::build(OrderState::S))
            .event(Event::build(OrderEvent::Payment))
            .source(State::build(OrderState::P))
            .target(State::build(OrderState::F))
            .event(Event::build(OrderEvent::Timeout))
            .trans_end()
            .end(State::build(OrderState::S))
            .build()
    }

    #[test]
    fn build() {
        let sm: StateMachine<OrderState, OrderEvent> = init_sm::<OrderState, OrderEvent>();
        let si = State::build(OrderState::I);
        assert!(sm.init.eq(&si));
        assert!(sm.current.eq(&si));
        assert_eq!(sm.trans.len(), 3);
    }

    #[test]
    fn send_event_normal() {
        let mut sm = init_sm::<OrderState, OrderEvent>();
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)), true);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)), false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Payment)), true);
        assert_eq!(sm.is_running(), false);
        assert_eq!(sm.get_state(), &State::build(OrderState::S));
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Payment)), false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Timeout)), false);
    }

    #[test]
    fn send_event_timeout() {
        let mut sm = init_sm::<OrderState, OrderEvent>();
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)), true);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)), false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Timeout)), true);
        assert_eq!(sm.is_running(), true);
        assert_eq!(sm.get_state(), &State::build(OrderState::F));
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Timeout)), false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Payment)), false);
    }
}
