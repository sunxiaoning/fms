use std::collections::HashMap;
use std::fmt::Debug;

struct TransitionBuilder<S: PartialEq + Debug + Clone, E: PartialEq> {
    source: Option<State<S>>,
    target: Option<State<S>>,
    event: Option<E>,
    action: Option<Action<S, E>>,
}

impl<S: PartialEq + Debug + Clone, E: PartialEq> TransitionBuilder<S, E> {
    fn build(&mut self) -> Transition<S, E> {
        Transition::new(
            self.source.take().expect("source state absent!"),
            self.target.take().expect("target state absent!"),
            self.event.take().expect("event absent"),
            self.action.take().expect("action absent"),
        )
    }
    fn action(&mut self, act: Action<S, E>) -> &mut Self {
        self.action = Some(act);
        self
    }
    fn event(&mut self, event: E) -> &mut Self {
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
            action: None,
        }
    }
}

struct StateContext<'a, S: PartialEq + Debug + Clone, E: PartialEq> {
    message: &'a Message<E>,
    tran: &'a Transition<S, E>,
}

impl<'a, S: PartialEq + Debug + Clone, E: PartialEq> StateContext<'a, S, E> {
    fn new(tran: &'a Transition<S, E>, message: &'a Message<E>) -> StateContext<'a, S, E> {
        StateContext { tran, message }
    }
}

type Action<S, E> = fn(ctx: &StateContext<S, E>) -> Result<bool, &'static str>;

struct Transition<S: PartialEq + Debug + Clone, E: PartialEq> {
    source: State<S>,
    target: State<S>,
    event: E,
    action: Action<S, E>,
}

impl<S: PartialEq + Debug + Clone, E: PartialEq> Transition<S, E> {
    fn transit(&self, _ctx: &StateContext<S, E>) -> Result<bool, &str> {
        Ok(true)
    }

    fn new(source: State<S>, target: State<S>, event: E, action: Action<S, E>) -> Self {
        Transition {
            source,
            target,
            event,
            action,
        }
    }
}

struct MessageBuilder<T> {
    payload: Option<T>,
    headers: Option<HashMap<String, String>>,
}

impl<T> MessageBuilder<T> {
    fn build(&mut self) -> Message<T> {
        let payload = self.payload.take().expect("payload absent!");
        if let None = self.headers {
            Message::new(payload)
        } else {
            Message::new_with_header(payload, self.headers.take().unwrap())
        }
    }

    fn add_header(&mut self, key: String, val: String) -> &mut MessageBuilder<T> {
        if let None = self.headers {
            self.headers = Some(HashMap::new());
        }
        self.headers.as_mut().unwrap().insert(key, val);
        self
    }

    fn payload(&mut self, payload: T) -> &mut MessageBuilder<T> {
        self.payload = Some(payload);
        self
    }

    fn new() -> MessageBuilder<T> {
        MessageBuilder {
            payload: None,
            headers: None,
        }
    }
}

struct Message<T> {
    payload: T,
    headers: Option<HashMap<String, String>>,
}

impl<T> Message<T> {
    fn get_header(&self, key: &str) -> Option<&String> {
        if let Some(headers) = &self.headers {
            headers.get(&String::from(key))
        } else {
            None
        }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }

    fn new(payload: T) -> Message<T> {
        Message {
            payload,
            headers: None,
        }
    }

    fn new_with_header(payload: T, headers: HashMap<String, String>) -> Message<T> {
        assert!(headers.len() > 0);
        Message {
            payload,
            headers: Some(headers),
        }
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
    fn action(self: Box<Self>, act: Action<S, E>) -> Box<StateMachineBuilder<S, E>>;
}

trait TransAdder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn and(self: Box<Self>) -> Box<dyn TranStarter<S, E>>;
}

trait TransEnder<S: PartialEq + Debug + Clone, E: PartialEq + Debug> {
    fn done(self: Box<Self>) -> Box<StateMachineBuilder<S, E>>;
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
            trans: Some(vec![]),
            trans_builder: None,
        }
    }
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

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> Target<S, E>
    for StateMachineBuilder<S, E>
{
    fn event(mut self: Box<Self>, event: E) -> Box<dyn Act<S, E>> {
        let tranb = self.trans_builder.as_mut().expect("trans builder absent");
        tranb.source.as_ref().expect("source absent");
        tranb.target.as_ref().expect("target absent");
        tranb.event(event);
        self
    }
}

impl<S: PartialEq + Debug + Clone, E: PartialEq + Debug> Act<S, E> for StateMachineBuilder<S, E> {
    fn action(mut self: Box<Self>, act: Action<S, E>) -> Box<StateMachineBuilder<S, E>> {
        let tranb = self.trans_builder.as_mut().expect("trans builder absent");
        tranb.source.as_ref().expect("source absent");
        tranb.target.as_ref().expect("target absent");
        tranb.event.as_ref().expect("event absent");
        tranb.action(act);
        self
    }
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> TransAdder<S, E>
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
}

impl<S: PartialEq + Debug + Clone + 'static, E: PartialEq + Debug + 'static> TransEnder<S, E>
    for StateMachineBuilder<S, E>
{
    fn done(mut self: Box<Self>) -> Box<StateMachineBuilder<S, E>> {
        self.trans
            .as_mut()
            .expect("trans absent!")
            .push(self.trans_builder.expect("trans builder absent!").build());
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
            if tran.source.eq(&self.current) && tran.event.eq(message.get_payload()) {
                let mut err_msg = None;
                let state_ctx = StateContext::new(tran, message);
                rs = tran.transit(&state_ctx).unwrap_or_else(|err| {
                    eprintln!(
                        "state machine trans: source: {:?}, event: {:?}, err: {}",
                        tran.source.id,
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
                (tran.action)(&state_ctx).unwrap();
                self.current = tran.target.clone();
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

    fn submit(ctx: &StateContext<OrderState, OrderEvent>) -> Result<bool, &'static str> {
        println!(
            "--- s: {:?}, t: {:?} submit!",
            ctx.tran.source.id, ctx.tran.target.id
        );
        let headers = ctx.message.headers.as_ref();
        if let Some(hs) = headers {
            println!("{:#?}", hs)
        } else {
            println!("none header!")
        }
        Ok(true)
    }

    fn pay(ctx: &StateContext<OrderState, OrderEvent>) -> Result<bool, &'static str> {
        println!(
            "---- s: {:?}, t: {:?} pay! ----",
            ctx.tran.source.id, ctx.tran.target.id
        );
        let headers = ctx.message.headers.as_ref();
        if let Some(hs) = headers {
            println!("{:#?}", hs);
            println!("nsk1: {:?}", ctx.message.get_header("nsk1"));
        } else {
            println!("none header!");
        }
        Ok(true)
    }

    fn err(ctx: &StateContext<OrderState, OrderEvent>) -> Result<bool, &'static str> {
        println!(
            "----- s: {:?}, t: {:?} timeout! -----",
            ctx.tran.source.id, ctx.tran.target.id
        );
        let headers = ctx.message.headers.as_ref();
        if let Some(hs) = headers {
            println!("{:#?}", hs);
        } else {
            println!("none header!");
        }
        Ok(true)
    }

    fn init_sm<S, E>() -> StateMachine<OrderState, OrderEvent> {
        Box::new(StateMachineBuilder::new())
            .init(State::build(OrderState::I))
            .trans()
            .source(State::build(OrderState::I))
            .target(State::build(OrderState::P))
            .event(OrderEvent::Submit)
            .action(submit)
            .and()
            .source(State::build(OrderState::P))
            .target(State::build(OrderState::S))
            .event(OrderEvent::Payment)
            .action(pay)
            .and()
            .source(State::build(OrderState::P))
            .target(State::build(OrderState::F))
            .event(OrderEvent::Timeout)
            .action(err)
            .done()
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
            true
        );
        assert_eq!(sm.is_running(), false);
        assert_eq!(sm.get_state(), &State::build(OrderState::S));
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
        assert_eq!(sm.get_state(), &State::build(OrderState::F));
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
