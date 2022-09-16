use std::fmt::Debug;

struct Transition<S: PartialEq+Clone,E: PartialEq> {
    source: State<S>,
    target: State<S>,
    event: Event<E>,
    // action: fn() ,
}

impl<S: PartialEq+Clone, E: PartialEq> Transition<S,E> {
    fn transit(&self) -> Result<bool, &str> {
        Ok(true)
    }

    fn build(source: State<S>, target: State<S>, event: Event<E>) -> Self {
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

impl<E:PartialEq> Event<E> {
    fn build(id: E) -> Self {
        Event { id }
    }
}


#[derive(Debug)]
struct State<S: PartialEq+Clone> {
    id: S,
}


impl<S: PartialEq+Clone> State<S> {

    fn build(id: S) -> Self {
        State { id }
    }
}

impl<S:PartialEq+Clone> PartialEq for State<S> {
    fn eq(&self, other: &State<S>) -> bool {
        self.id == other.id
    }
}

impl<S:PartialEq+Clone> Clone for State<S> {
    fn clone(&self) -> Self {
       State { id: self.id.clone()}
    }
}

struct StateMachine<S: PartialEq+Debug+Clone,E:PartialEq+Debug> {
    init: State<S>,
    end: State<S>,
    current: State<S>,
    err: Option<String>,
    trans: Vec<Transition<S,E>>,
}

impl<'a,S: PartialEq+Debug+Clone,E:PartialEq+Debug> StateMachine<S,E> {
    fn is_running(&self) -> bool {
        return !self.current.eq(&self.end)
    }

    fn has_err(&self) -> bool {
        match self.err {
            Some(_) => true,
            None => false,
        }
    }

    fn get_state(&self) ->&State<S> {
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
                rs = trans.transit().unwrap_or_else(|err|  {
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

    fn build(init: State<S>, end: State<S>, trans: Vec<Transition<S,E>>) -> StateMachine<S, E> {
        let sm = StateMachine {
            init: init.clone(),
            end,
            current: init,
            trans,
            err: None,
        };
        sm
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[derive(PartialEq,Debug,Clone)]
enum OrderState {
        I,
        P,
        S,
        F
    }

    #[derive(PartialEq,Debug)]
    enum OrderEvent {
        Submit,
        Payment,
        Timeout
    }

    fn init_sm<S,E>() -> StateMachine<OrderState,OrderEvent> {
        let si = State::build(OrderState::I);
        let sp = State::build(OrderState::P);
        let ss = State::build(OrderState::S);
        let sf = State::build(OrderState::F);

        let es = Event::build(OrderEvent::Submit);
        let ep = Event::build(OrderEvent::Payment);
        let et = Event::build(OrderEvent::Timeout);

        StateMachine::build(
            si.clone(),
            ss.clone(),
            vec![
                Transition::build(si, sp.clone(), es),
                Transition::build(sp.clone(), ss, ep),
                Transition::build(sp, sf, et),
            ],
        )
    }

    #[test]
    fn build() {
        let sm: StateMachine<OrderState, OrderEvent> = init_sm::<OrderState,OrderEvent>();
        let si = State::build(OrderState::I);
        assert!(sm.init.eq(&si));
        assert!(sm.current.eq(&si));
        assert_eq!(sm.trans.len(), 3);
    }

    #[test]
    fn send_event_normal() {
        let mut sm = init_sm::<OrderState,OrderEvent>();
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)),true);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)),false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Payment)),true);
        assert_eq!(sm.is_running(), false);
        assert_eq!(sm.get_state(),&State::build(OrderState::S));
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Payment)),false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Timeout)),false);
    }

    #[test]
    fn send_event_timeout() {
        let mut sm = init_sm::<OrderState,OrderEvent>();
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)),true);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Submit)),false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Timeout)),true);
        assert_eq!(sm.is_running(), true);
        assert_eq!(sm.get_state(),&State::build(OrderState::F));
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Timeout)),false);
        assert_eq!(sm.send_event(&Event::build(OrderEvent::Payment)),false);

    }
}
