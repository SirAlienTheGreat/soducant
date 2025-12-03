pub mod solver {
    use rand::Rng;

    use crate::sudoku::sudoku::{Board, Solution};

    use std::time::Instant;

    pub struct Solver {
        pub temperature: f64,
        pub decay: f64,
        pub board: Board,
        pub current_score: u32,
        pub solution: Solution,
        time_since_last_improvement: u32,
    }

    impl Default for Solver {
        fn default() -> Self {
            let b = Board::from_string(
                "000057001751094000204000073400610009500920810900408365020709100108040030600000207",
            );
            let s = Solution::new_from_board(&b).unwrap();
            Solver {
                temperature: 9.0,
                decay: 0.980,
                current_score: b.score_solution(&s).unwrap(),
                solution: s,
                board: b,
                time_since_last_improvement: 0,
            }
        }
    }

    impl Solver {
        pub fn new_with_params(board: &str, temperature: f64, decay: f64) -> Self {
            let mut solver = Self::default();
            solver.temperature = temperature;
            solver.decay = decay;
            solver.board = Board::from_string(board);
            solver.solution = Solution::new_from_board(&solver.board).unwrap();
            solver.current_score = solver.board.score_solution(&solver.solution).unwrap();
            return solver;
        }

        pub fn new_with_board(board: &str) -> Self {
            let mut solver = Self::default();
            solver.board = Board::from_string(board);
            solver.solution = Solution::new_from_board(&solver.board).unwrap();
            solver.current_score = solver.board.score_solution(&solver.solution).unwrap();
            return solver;
        }

        fn cool(&mut self) {
            self.temperature = self.temperature * self.decay;
        }

        fn solve_one_step(&mut self) -> Option<Solution> {
            let mut new_solution = self.solution.clone();
            new_solution.flip_random_spaces();

            let new_score = self.board.score_solution(&new_solution).unwrap();

            // if new is better than old, accept it
            if new_score < self.current_score {
                /*println!(
                    "accepting new move: (Current: {}, new: {})",
                    self.current_score, new_score
                );*/
                self.solution = new_solution;
                self.current_score = new_score;
                self.time_since_last_improvement = 0;
                if self.current_score == 0 {
                    return Some(self.solution.clone());
                } else {
                    return None;
                }
            }
            self.time_since_last_improvement += 1;
            // otherwise, accept it only randomly
            let p_accept = (0.0
                - ((new_score as i32 - self.current_score as i32) as f64 / self.temperature))
                .exp();
            self.cool();
            //println!("P(accept)={}", p_accept);

            let mut rng = rand::rng();
            if rng.random_bool(p_accept) {
                /*println!(
                    "New solution randomly selected: (Current: {}, new: {})",
                    self.current_score, new_score
                );*/
                self.solution = new_solution;
                self.current_score = new_score;
                //return Some(self.solution.clone());
            }
            return None;
        }

        pub fn solve(&mut self, max_iter: i32) -> Option<Solution> {
            for _ in 0..max_iter {
                let step = self.solve_one_step();
                /*println!(
                    "Solution after {} steps (score {}):\n{}",
                    i, self.current_score, self.board
                );*/
                if let Some(solution) = step {
                    //println!("Solved after {} iterations", i);
                    return Some(solution);
                }
                if self.time_since_last_improvement >= 750 {
                    for _ in 0..100 {
                        self.solution.flip_random_spaces();
                    }
                    self.current_score = self.board.score_solution(&self.solution).unwrap();
                }
            }
            return None;
        }
    }

    pub fn benchmark() {
        let mut passes = 0;
        let mut attempts = 0;

        let start = Instant::now();

        for line in include_str!("test_sudokus.txt").lines() {
            let mut solver = Solver::new_with_board(line);
            let solution = solver.solve(20000);
            if solution.is_some() {
                passes += 1;
                //println!("solution is: \n{}", solution.unwrap());
            }
            attempts += 1;
        }

        let duration = start.elapsed();

        println!(
            "Benchmarking results: {}/{} ({}%) passes",
            passes,
            attempts,
            100.0 * passes as f32 / attempts as f32
        );
        println!("Time taken: {:?}", duration); // Prints in a readable format
    }
}
