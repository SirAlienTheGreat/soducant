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

#[wasm_bindgen]
pub fn benchmark_intern() {
    for _ in 0..1000 {
        soducant_core::solver::solver::solve(&soducant_core::solver::solver::get_random_sudoku())
            .unwrap_or_else(|_| {
                println!("Couldn't solve sudoku");
                "Error".to_string()
            });
    }
}
