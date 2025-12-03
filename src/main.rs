mod sudoku;
use crate::solver::solver::Solver;
use crate::solver::solver::benchmark;
use crate::sudoku::sudoku::Board;

mod solver;

fn main() {
    let board = Board::from_string(
        "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
    );
    //let solution = Solution::new_from_board(&board).unwrap(); // Solution::from_board(Board::from_string(
    //"000395060040082307310740028002050716080210549500467032954608000003570290020000000",
    //));

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

    benchmark();

    /*let mut passes = 0;
    let attempts = 500;

    for _ in 0..attempts {
        let mut solver = Solver::default();
        let solution = solver.solve(20000);
        if solution.is_some() {
            passes += 1;
        }
    }

    println!(
        "Benchmarking results: {}/{} ({}%) passes",
        passes,
        attempts,
        100.0 * passes as f32 / attempts as f32
    );*/

    // 278000401609100050005006900430809000706003000091000800000020173860001004107934685

    //let score = board.score_solution(&solution).unwrap();

    //println!("{}", score);
}
