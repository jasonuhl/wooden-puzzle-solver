mod solver;

use std::sync::Mutex;

use wasm_bindgen::prelude::*;

pub use solver::{stream_solutions, SolverIterator};

static SOLVER_STATE: Mutex<Option<SolverIterator>> = Mutex::new(None);

#[wasm_bindgen]
pub fn next_solution() -> String {
    let mut guard = SOLVER_STATE.lock().unwrap();

    let iter = guard.get_or_insert_with(SolverIterator::new);
    iter.next().unwrap_or_default()
}
