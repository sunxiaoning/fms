pub mod msg;
pub mod state;
pub mod trans;
use crate::sm::state::*;
use crate::sm::trans::*;

use crate::sm::msg::*;

use std::fmt::Debug;

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

    fn send_event(&mut self, message: &Message<E>) -> bool {
        if self.has_err() {
            println!("statemachine err: {}!", self.err.as_ref().unwrap());
            return false;
        }
        if !self.is_running() {
            println!(
                "statemachine not running, event: {:?} not accept!",
                message.get_payload()
            );
            return false;
        }

        let mut rs = false;
        for tran in self.trans.iter() {
            if tran.source().eq(&self.current) && tran.event().eq(message.get_payload()) {
                let mut err_msg = None;
                let state_ctx = StateContext::new(tran, message);
                rs = tran.transit(&state_ctx).unwrap_or_else(|err| {
                    eprintln!(
                        "state machine trans: source: {:?}, event: {:?}, err: {}",
                        tran.source().id(),
                        message.get_payload(),
                        err
                    );
                    err_msg = Some(String::from(err.clone()));
                    false
                });
                self.err = err_msg;
                if !rs {
                    println!(
                        "statemachine event: {:?} not accept!",
                        message.get_payload()
                    );
                    break;
                }
                if let Some(act) = tran.action() {
                    act(&state_ctx).unwrap();
                }
                self.current = tran.target().clone();
                break;
            }
        }
        println!(
            "statemachine event: {:?} not accept!",
            message.get_payload()
        );
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

struct StateMachineBuilder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    init: Option<State<S>>,
    end: Option<State<S>>,
    trans: Option<Vec<Transition<S, E>>>,
    trans_builder: Option<TransitionBuilder<S, E>>,
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static>
    StateMachineBuilder<S, E>
{
    fn config(self) -> Box<dyn SmBuilder<S, E>> {
        Box::new(self)
    }
    fn new() -> StateMachineBuilder<S, E> {
        StateMachineBuilder {
            init: None,
            end: None,
            trans: Some(vec![]),
            trans_builder: None,
        }
    }
}
trait SmBuilder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn init(self: Box<Self>, init: State<S>) -> Box<dyn TransBuilder<S, E>>;
}

trait TransBuilder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn trans(self: Box<Self>) -> Box<dyn TranStarter<S, E>>;
}

trait TranStarter<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn source(self: Box<Self>, source: State<S>) -> Box<dyn Source<S, E>>;
}

trait Source<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn target(self: Box<Self>, target: State<S>) -> Box<dyn Target<S, E>>;
}

trait Target<S: PartialEq + Debug + Debug + Clone, E: PartialEq + Debug> {
    fn event(self: Box<Self>, event: E) -> Box<dyn Act<S, E>>;
}

trait Act<S: PartialEq + Debug + Debug + Clone, E: PartialEq + Debug> {
    fn action(self: Box<Self>, act: Option<Action<S, E>>) -> Box<dyn Gurd<S, E>>;
}

trait Gurd<S: PartialEq + Debug + Debug + Clone, E: PartialEq + Debug> {
    fn guard(self: Box<Self>, guard: Option<Guard<S, E>>) -> Box<dyn TransEnder<S, E>>;
}

trait TransEnder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn and(self: Box<Self>) -> Box<dyn TranStarter<S, E>>;
    fn done(self: Box<Self>) -> Box<dyn SmEnder<S, E>>;
}

trait SmEnder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn end(self: Box<Self>, end: State<S>) -> Box<dyn SmFactory<S, E>>;
}

trait SmFactory<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn build(self: Box<Self>) -> StateMachine<S, E>;
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> SmBuilder<S, E>
    for StateMachineBuilder<S, E>
{
    fn init(mut self: Box<Self>, init: State<S>) -> Box<dyn TransBuilder<S, E>> {
        self.init = Some(init);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> TransBuilder<S, E>
    for StateMachineBuilder<S, E>
{
    fn trans(mut self: Box<Self>) -> Box<dyn TranStarter<S, E>> {
        self.trans_builder = Some(TransitionBuilder::new());
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> TranStarter<S, E>
    for StateMachineBuilder<S, E>
{
    fn source(mut self: Box<Self>, source: State<S>) -> Box<dyn Source<S, E>> {
        self.trans_builder
            .as_mut()
            .expect("trans builder absent")
            .source(source);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> Source<S, E>
    for StateMachineBuilder<S, E>
{
    fn target(mut self: Box<Self>, target: State<S>) -> Box<dyn Target<S, E>> {
        self.trans_builder
            .as_mut()
            .expect("trans builder absent")
            .target(target);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> Target<S, E>
    for StateMachineBuilder<S, E>
{
    fn event(mut self: Box<Self>, event: E) -> Box<dyn Act<S, E>> {
        self.trans_builder
            .as_mut()
            .expect("trans builder absent")
            .event(event);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> Act<S, E>
    for StateMachineBuilder<S, E>
{
    fn action(mut self: Box<Self>, act: Option<Action<S, E>>) -> Box<dyn Gurd<S, E>> {
        self.trans_builder
            .as_mut()
            .expect("trans builder absent")
            .action(act);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> Gurd<S, E>
    for StateMachineBuilder<S, E>
{
    fn guard(mut self: Box<Self>, guard: Option<Guard<S, E>>) -> Box<dyn TransEnder<S, E>> {
        self.trans_builder
            .as_mut()
            .expect("trans builder absent")
            .guard(guard);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> TransEnder<S, E>
    for StateMachineBuilder<S, E>
{
    fn and(mut self: Box<Self>) -> Box<dyn TranStarter<S, E>> {
        self.trans
            .as_mut()
            .expect("trans absent!")
            .push(self.trans_builder.expect("trans builder absent!").build());
        self.trans_builder = Some(TransitionBuilder::new());
        self
    }
    fn done(mut self: Box<Self>) -> Box<dyn SmEnder<S, E>> {
        self.trans
            .as_mut()
            .expect("trans absent!")
            .push(self.trans_builder.expect("trans builder absent!").build());
        self.trans_builder = None;
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> SmEnder<S, E>
    for StateMachineBuilder<S, E>
{
    fn end(mut self: Box<Self>, end: State<S>) -> Box<dyn SmFactory<S, E>> {
        self.init.as_ref().expect("init absent!");
        assert!(self.trans.as_ref().expect("trans absent!").len() > 0);
        self.end = Some(end);
        self
    }
}

impl<S: PartialEq + Debug + Clone, E: PartialEq + Debug> SmFactory<S, E>
    for StateMachineBuilder<S, E>
{
    fn build(mut self: Box<Self>) -> StateMachine<S, E> {
        StateMachine::new(
            self.init.take().expect("init state absent!"),
            self.end.take().expect("end state absent!"),
            self.trans.take().expect("trans absent!"),
        )
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

    fn submit(ctx: &StateContext<OrderState, OrderEvent>) -> Result<(), &'static str> {
        println!(
            "--- s: {:?}, t: {:?} submit!",
            ctx.tran().source().id(),
            ctx.tran().target().id()
        );
        let headers = ctx.message().headers();
        if let Some(hs) = headers {
            println!("{:#?}", hs)
        } else {
            println!("none header!")
        }
        Ok(())
    }

    fn pay(ctx: &StateContext<OrderState, OrderEvent>) -> Result<(), &'static str> {
        println!(
            "---- s: {:?}, t: {:?} pay! ----",
            ctx.tran().source().id(),
            ctx.tran().target().id()
        );
        let headers = ctx.message().headers();
        if let Some(hs) = headers {
            println!("{:#?}", hs);
            println!("nsk1: {:?}", ctx.message().get_header("nsk1"));
        } else {
            println!("none header!");
        }
        Ok(())
    }

    fn err(ctx: &StateContext<OrderState, OrderEvent>) -> Result<(), &'static str> {
        println!(
            "----- s: {:?}, t: {:?} timeout! -----",
            ctx.tran().source().id(),
            ctx.tran().target().id()
        );
        let headers = ctx.message().headers();
        if let Some(hs) = headers {
            println!("{:#?}", hs);
        } else {
            println!("none header!");
        }
        Ok(())
    }

    fn init_sm<S, E>() -> StateMachine<OrderState, OrderEvent> {
        StateMachineBuilder::new()
            .config()
            .init(State::new(OrderState::I))
            .trans()
            .source(State::new(OrderState::I))
            .target(State::new(OrderState::P))
            .event(OrderEvent::Submit)
            .action(Some(submit))
            .guard(None)
            .and()
            .source(State::new(OrderState::P))
            .target(State::new(OrderState::S))
            .event(OrderEvent::Payment)
            .action(Some(pay))
            .guard(Some(|ctx| {
                if let Some(hres) = ctx.message().get_header("hres") {
                    hres.to_string() == "ok"
                } else {
                    false
                }
            }))
            .and()
            .source(State::new(OrderState::P))
            .target(State::new(OrderState::F))
            .event(OrderEvent::Timeout)
            .action(Some(err))
            .guard(None)
            .done()
            .end(State::new(OrderState::S))
            .build()
    }

    #[test]
    fn build() {
        let sm: StateMachine<OrderState, OrderEvent> = init_sm::<OrderState, OrderEvent>();
        let si = State::new(OrderState::I);
        assert!(sm.init.eq(&si));
        assert!(sm.current.eq(&si));
        assert_eq!(sm.trans.len(), 3);
    }

    #[test]
    fn send_event_normal() {
        let mut sm = init_sm::<OrderState, OrderEvent>();
        assert_eq!(
            sm.send_event(
                &MessageBuilder::new()
                    .payload(OrderEvent::Submit)
                    .add_header("nsk1".to_string(), "nsv1".to_string())
                    .build()
            ),
            true
        );
        assert_eq!(
            sm.send_event(&MessageBuilder::new().payload(OrderEvent::Submit).build()),
            false
        );
        assert_eq!(
            sm.send_event(
                &MessageBuilder::new()
                    .payload(OrderEvent::Payment)
                    .add_header("npk1".to_string(), "npv1".to_string())
                    .build()
            ),
            false
        );
        assert_eq!(
            sm.send_event(
                &MessageBuilder::new()
                    .payload(OrderEvent::Payment)
                    .add_header("npk1".to_string(), "npv1".to_string())
                    .add_header("hres".to_string(), "ook".to_string())
                    .build()
            ),
            false
        );
        assert_eq!(
            sm.send_event(
                &MessageBuilder::new()
                    .payload(OrderEvent::Payment)
                    .add_header("npk1".to_string(), "npv1".to_string())
                    .add_header("hres".to_string(), "ok".to_string())
                    .build()
            ),
            true
        );
        assert_eq!(sm.is_running(), false);
        assert_eq!(sm.get_state(), &State::new(OrderState::S));
        assert_eq!(
            sm.send_event(&MessageBuilder::new().payload(OrderEvent::Payment).build()),
            false
        );
        assert_eq!(
            sm.send_event(&MessageBuilder::new().payload(OrderEvent::Timeout).build()),
            false
        );
    }

    #[test]
    fn send_event_timeout() {
        let mut sm = init_sm::<OrderState, OrderEvent>();
        assert_eq!(
            sm.send_event(
                &MessageBuilder::new()
                    .payload(OrderEvent::Submit)
                    .add_header("tpk1".to_string(), "tpv1".to_string())
                    .build()
            ),
            true
        );
        assert_eq!(
            sm.send_event(&MessageBuilder::new().payload(OrderEvent::Submit).build()),
            false
        );
        assert_eq!(
            sm.send_event(
                &MessageBuilder::new()
                    .payload(OrderEvent::Timeout)
                    .add_header("tpk1".to_string(), "tpv1".to_string())
                    .build()
            ),
            true
        );
        assert_eq!(sm.is_running(), true);
        assert_eq!(sm.get_state(), &State::new(OrderState::F));
        assert_eq!(
            sm.send_event(&MessageBuilder::new().payload(OrderEvent::Timeout).build()),
            false
        );
        assert_eq!(
            sm.send_event(&MessageBuilder::new().payload(OrderEvent::Payment).build()),
            false
        );
    }
}
