use std::{fmt::Debug, hash::Hash};

pub trait StateId: PartialEq + Debug + Clone + Eq + Hash {}

pub struct State<S: StateId> {
    id: S,
}

impl<S: StateId> State<S> {
    pub fn new(id: S) -> Self {
        State { id }
    }

    pub fn id(&self) -> &S {
        &self.id
    }
}

impl<S: StateId> PartialEq for State<S> {
    fn eq(&self, other: &State<S>) -> bool {
        self.id == other.id
    }
}

impl<S: StateId> Eq for State<S> {}

impl<S: StateId> Hash for State<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<S: StateId> Debug for State<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").field("id", &self.id).finish()
    }
}

impl<S: StateId> Clone for State<S> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
        }
    }
}
