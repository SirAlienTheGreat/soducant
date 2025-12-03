pub mod solver {
    use rand::Rng;

    use crate::sudoku::sudoku::{Board, Solution};

    pub struct Solver {
        pub temperature: f64,
        pub decay: f64,
        pub board: Board,
        pub current_score: u32,
        pub solution: Solution,
    }

    impl Default for Solver {
        fn default() -> Self {
            let b = Board::from_string(
                "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
            );
            let s = Solution::new_from_board(&b).unwrap();
            Solver {
                temperature: 9.0,
                decay: 0.99,
                current_score: b.score_solution(&s).unwrap(),
                solution: s,
                board: b,
            }
        }
    }

    impl Solver {
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
                if self.current_score == 0 {
                    return Some(self.solution.clone());
                } else {
                    return None;
                }
            }

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
            for i in 0..max_iter {
                let step = self.solve_one_step();
                /*println!(
                    "Solution after {} steps (score {}):\n{}",
                    i, self.current_score, self.board
                );*/
                if let Some(solution) = step {
                    //println!("Solved after {} iterations", i);
                    return Some(solution);
                }
            }
            return None;
        }
    }
}
