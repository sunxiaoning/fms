#[derive(Debug)]
pub struct State<S: PartialEq + Clone> {
    id: S,
}

impl<S: PartialEq + Clone> State<S> {
    pub fn new(id: S) -> Self {
        State { id }
    }

    pub fn id(&self) -> &S {
        &self.id
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
