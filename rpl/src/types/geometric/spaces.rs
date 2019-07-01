use itertools::{multizip, Itertools};

/// Geometric State Spaces:
/// This module defines the general state space and state interfaces, as well as implementations for
/// several important/common geometric state spaces

/// A trait representing a generic state in a geometric state space. Implementers are expected to
/// hold a reference to the state space instance for a given state; that's about the only
/// requirement. This interface ties a state type to its corresponding state space. States must also
/// be copyable
trait State: Copy {
    fn set_state_space(&mut self, space: &StateSpace<StateT = Self>);
    fn get_state_space(&self) -> &StateSpace<StateT = Self>;
    fn new(space: &StateSpace<StateT = Self>) -> State;
}

/// A trait representing a generic geometric state space. The associated type `StateT` is the state
/// representation for the state space; it must implement `State`.
trait StateSpace {
    type StateT: State;
    fn distance(&self, a: &Self::StateT, b: &Self::StateT) -> f64;
    fn interpolate(&self, from: &Self::StateT, to: &Self::StateT, step: f64) -> Self::StateT;
    fn interpolate_into(
        &self,
        from: &Self::StateT,
        to: &Self::StateT,
        step: f64,
        result: &mut Self::StateT,
    );
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn contains<T: StateSpace>(&self, space: &T) -> bool;
    fn covers<T: StateSpace>(&self, space: &T) -> bool;
    fn set_segment_length(&mut self, step: f64);
    fn get_segment_length(&self) -> f64;
    fn count_segments_between(&self, a: &Self::StateT, b: &Self::StateT) -> isize;
}

#[derive(Copy)]
struct CompoundState<'a> {
    values: Vec<Box<State>>,
    space: &'a StateSpace<StateT = Self>,
}

impl State for CompoundState<'a> {
    fn new(space: &'a StateSpace<StateT = Self>) -> Self {
        Self {
            values: Vec::new(),
            space: space,
        }
    }

    fn set_state_space(&mut self, space: &'a StateSpace<StateT = Self>) {
        self.space = space;
    }

    fn get_state_space(&self) -> &'a StateSpace<StateT = Self> {
        self.space
    }
}

struct CompoundStateSpace {
    name: String,
    components: Vec<Box<StateSpace>>,
    segment_length: f64,
}

impl StateSpace for CompoundStateSpace {
    type StateT = CompoundState;
    fn distance(&self, a: &Self::StateT, b: &Self::StateT) -> f64 {
        multizip((&self.components, &a.values, &b.values))
            .fold(0.0, |acc, (&subspace, &a_sub, &b_sub)| {
                acc + subspace.distance(a_sub, b_sub)
            })
    }

    fn interpolate(&self, from: &Self::StateT, to: &Self::StateT, step: f64) -> Self::StateT {
        let mut result = Self::StateT::new();
        self.interpolate_into(from, to, step, &mut result);
        result
    }

    fn interpolate_into(
        &self,
        from: &Self::StateT,
        to: &Self::StateT,
        step: f64,
        result: &mut Self::StateT,
    ) {
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn contains<T: StateSpace>(&self, space: &T) -> bool {
        // NOTE: This does not attempt to make  the state space by combinations of subspaces - this
        // seems infeasible to do, but I should check what OMPL does here
        self.components
            .iter()
            .any(|&subspace| subspace.contains(space))
    }

    fn covers<T: StateSpace>(&self, space: &T) -> bool {
        self.components
            .iter()
            .any(|&subspace| subspace.contains(space))
    }

    fn set_segment_length(&mut self, step: f64) {
        self.segment_length = step;
    }

    fn get_segment_length(&self) -> f64 {
        self.segment_length
    }

    fn count_segments_between(&self, a: &Self::StateT, b: &Self::StateT) -> isize {}
}
