trait State {
  
}

trait StateSpace {
   type State: State;
   pub fn distance(a: State, b: State) -> f64;
   pub fn interpolate(from: State, to: State) -> State;
   pub fn interpolate(from: State, to: State, result: &mut State);
}
