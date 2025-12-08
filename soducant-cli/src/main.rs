use soducant_core::solver;
use soducant_core::solver::solver::Solver;
use soducant_core::sudoku;
use soducant_core::sudoku::sudoku::Board;
use soducant_core::sudoku::sudoku::Solution;
use std::env;

fn main() {
    if let Some(cmd) = env::args().nth(1) {
        if cmd.to_lowercase() == "benchmark"
            || cmd.to_lowercase() == "--benchmark"
            || cmd.to_lowercase() == "-benchmark"
        {
            println!("Running benchmark");
            solver::solver::benchmark(env::args().nth(2).is_some());
        } else {
            if let Ok(board) = sudoku::sudoku::Board::from_string(&cmd) {
                println!("Solving the following sudoku:\n{}", board);
                if let Ok(solution) = solver::solver::solve(&cmd) {
                    println!(
                        "The solved board is:\n{}",
                        Solution::from_board(Board::from_string(&solution).unwrap())
                    );
                } else {
                    println!("Couldn't solve sudoku");
                }
            } else {
                println!("Couldn't read sudoku from input");
            }
        }
        return;
    }

    println!(
        "No CLI input found, using default board and solving (pass a board by string or use --benchmark to use other modes)"
    );
    let board = Board::from_string(
        "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
    )
    .unwrap();

    println!("Board:\n{}", board);

    println!("Solving sudoku");

    let mut solver = Solver::default();
    let solution = solver.solve(10000);

    if let Some(solved) = solution {
        println!("Solution:\n{}", solved);
    } else {
        println!(
            "Couldn't solve sudoku. Ended with score {} and board:\n{}",
            solver.current_score, solver.solution
        )
    }
}
