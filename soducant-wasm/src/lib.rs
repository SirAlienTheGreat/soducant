//use soducant_core::solver;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(puzzle: &str) -> String {
    return soducant_core::solver::solver::solve(puzzle).unwrap_or("Error".to_string());
}

#[wasm_bindgen]
pub fn get_random_puzzle() -> String {
    return soducant_core::solver::solver::get_random_sudoku();
}
