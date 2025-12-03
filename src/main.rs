mod sudoku;
use crate::sudoku::sudoku::Board;
use crate::sudoku::sudoku::Solution;

fn main() {
    let board = Board::from_string(
        "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
    );
    let solution = Solution::new_from_board(&board).unwrap(); // Solution::from_board(Board::from_string(
        //"000395060040082307310740028002050716080210549500467032954608000003570290020000000",
    //));

    println!("Board:\n{}", board);
    println!("Solution:\n{}", solution);
    // 278000401609100050005006900430809000706003000091000800000020173860001004107934685

    let score = board.score_solution(&solution).unwrap();

    println!("{}", score);
}
