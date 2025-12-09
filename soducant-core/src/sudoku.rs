pub mod sudoku {
    use bit_vec::BitVec;
    use rand::prelude::*;
    use std::{
        fmt::{Display, Formatter},
        ptr,
    };

    const BOARD_SIZE: usize = 9;

    #[derive(Debug, Clone)]
    pub struct Board {
        pub(crate) grid: [[Option<i8>; BOARD_SIZE]; BOARD_SIZE],
    }

    impl Board {
        pub fn from_arr_with_zeros<T: Into<i8> + Clone>(
            arr: [[T; BOARD_SIZE]; BOARD_SIZE],
        ) -> Self {
            let mut grid: [[Option<i8>; BOARD_SIZE]; BOARD_SIZE] = [[None; BOARD_SIZE]; BOARD_SIZE];
            for (i, _) in arr.iter().enumerate() {
                for (j, _) in arr[i].iter().enumerate() {
                    let val = arr[i][j].clone().into();
                    if val != 0i8 {
                        grid[i][j] = Some(val.into());
                    }
                }
            }

            return Self { grid };
        }

        pub fn to_clean_string(&self) -> String {
            let mut out = "".to_string();
            for i in 0..9 {
                for j in 0..9 {
                    out += &self.grid[i][j].unwrap_or(0).to_string();
                }
            }
            return out;
        }

        pub fn from_string(str: &str) -> Result<Self, Box<dyn std::error::Error>> {
            if str.len() != BOARD_SIZE * BOARD_SIZE {
                return Err(Box::from(
                    "Board construction from string requires properly sized board",
                ));
            }

            let mut arr: [[i8; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];

            for (i, c) in str.chars().enumerate() {
                /*assert!(
                    c.is_ascii_digit(),
                    "String entry contains a non-number {}",
                    c
                );*/
                arr[i / BOARD_SIZE][i % BOARD_SIZE] = c
                    .to_digit(10)
                    .ok_or("Board construction from string requires only numbers in string")?
                    as i8;
            }

            return Ok(Self::from_arr_with_zeros(arr));
        }

        pub fn from_solution(board: Solution) -> Self {
            Self { grid: board.grid }
        }

        pub fn combine(&self, other: &Solution) -> Result<Board, Box<dyn std::error::Error>> {
            let mut combined = Board::default();

            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    match (self.grid[i][j], other.grid[i][j]) {
                        (None, None) => {
                            return Err(Box::from(format!(
                                "Neither board nor solution have ({},{})",
                                i, j
                            )));
                        }
                        (None, Some(num)) => {
                            combined.grid[i][j] = Some(num);
                        }
                        (Some(num), None) => {
                            combined.grid[i][j] = Some(num);
                        }
                        (Some(_), Some(_)) => {
                            return Err(Box::from(format!(
                                "Both board and solution have ({},{})",
                                i, j
                            )));
                        }
                    }
                }
            }

            return Ok(combined);
        }

        pub fn score_solution(&self, other: &Solution) -> Result<u32, Box<dyn std::error::Error>> {
            let mut score: u32 = 0;

            let combined = Self::combine(&self, other)?;

            // Rows
            for x in combined.grid {
                let mut bv = BitVec::from_elem(BOARD_SIZE, false);
                for i in x {
                    bv.set(unsafe { i.unwrap_unchecked() - 1 } as usize, true);
                }
                score += bv.count_zeros() as u32;
            }

            // Columns
            for i in 0..BOARD_SIZE {
                let mut bv = BitVec::from_elem(BOARD_SIZE, false);
                for j in 0..BOARD_SIZE {
                    bv.set(
                        unsafe { combined.grid[j][i].unwrap_unchecked() - 1 } as usize,
                        true,
                    );
                }
                score += bv.count_zeros() as u32;
            }

            // Blocks
            for block_i in 0..3 {
                for block_j in 0..3 {
                    let mut bv = BitVec::from_elem(BOARD_SIZE, false);
                    for i in 0..3 {
                        for j in 0..3 {
                            bv.set(
                                unsafe {
                                    combined.grid[block_i * 3 + i][block_j * 3 + j]
                                        .unwrap_unchecked()
                                        - 1
                                } as usize,
                                true,
                            );
                        }
                    }
                    score += bv.count_zeros() as u32;
                }
            }

            return Ok(score);
        }
    }

    impl Default for Board {
        fn default() -> Self {
            Self {
                grid: [[None; BOARD_SIZE]; BOARD_SIZE],
            }
        }
    }

    //Display assumes 9x9 board
    impl Display for Board {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "╔═══════════════════════╗\n")?;
            for (i, x) in self.grid.iter().enumerate() {
                if i == 3 || i == 6 {
                    write!(f, "║⋯⋯⋯⋯⋯⋯⋯⸭⋯⋯⋯⋯⋯⋯⋯⸭⋯⋯⋯⋯⋯⋯⋯║\n")?;
                }
                write!(f, "║")?;

                for (j, y) in x.iter().enumerate() {
                    if j == 3 || j == 6 {
                        write!(f, " ⁞")?;
                    }
                    match y {
                        Some(num) => write!(f, " {}", num)?,
                        None => write!(f, "  ")?,
                    }
                }
                write!(f, " ║\n")?;
            }
            write!(f, "╚═══════════════════════╝ \n")?;

            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub struct Solution {
        pub(crate) grid: [[Option<i8>; BOARD_SIZE]; BOARD_SIZE],
    }

    impl Solution {
        pub fn from_board(board: Board) -> Self {
            Self { grid: board.grid }
        }

        pub fn new_from_board(board: &Board) -> Result<Self, Box<dyn std::error::Error>> {
            let mut solution = Solution::default();

            for block_i in 0..3 {
                for block_j in 0..3 {
                    // for each block, find the unused numbers ...
                    let mut taken = [false; 9];
                    for i in 0..3 {
                        for j in 0..3 {
                            let tile = board.grid[block_i * 3 + i][block_j * 3 + j];
                            if let Some(num) = tile {
                                if taken[(num - 1) as usize] {
                                    return Err(Box::from(format!(
                                        "{} is used twice in same original board block ({},{})",
                                        num,
                                        block_i * 3 + i,
                                        block_j * 3 + j
                                    )));
                                }
                                taken[(num - 1) as usize] = true;
                            }
                        }
                    }
                    // ... and add them in order to the unfilled spots
                    for i in 0..3 {
                        for j in 0..3 {
                            let tile = board.grid[block_i * 3 + i][block_j * 3 + j];
                            if let Some(_) = tile {
                            } else {
                                solution.grid[block_i * 3 + i][block_j * 3 + j] =
                                    Some(find_next_empty_num(&mut taken));
                            }
                        }
                    }
                }
            }

            return Ok(solution);
        }

        pub fn to_clean_string(&self) -> String {
            let mut out = "".to_string();
            for i in 0..9 {
                for j in 0..9 {
                    out += &self.grid[i][j].unwrap_or(0).to_string();
                }
            }
            return out;
        }

        pub(crate) fn flip_spaces(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
            unsafe {
                let pa: *mut Option<i8> = &mut self.grid[x1][y1];
                let pb: *mut Option<i8> = &mut self.grid[x2][y2];
                ptr::swap(pa, pb);
            }
        }

        //todo: maybe use faster random algorithm
        pub fn flip_random_spaces_in_block(
            &mut self,
            block_x: usize,
            block_y: usize,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut rng = rand::rng();

            let mut available_spaces = [(99, 99); 9];
            let mut num_spaces = 0;

            for i in 0..3 {
                for j in 0..3 {
                    if self.grid[block_x * 3 + i][block_y * 3 + j].is_some() {
                        available_spaces[num_spaces] = (block_x * 3 + i, block_y * 3 + j);
                        num_spaces += 1;
                    }
                }
            }

            if num_spaces == 0 {
                return Err(Box::from("No spaces in selected block"));
            }

            let space_1_i = rng.random_range(0..num_spaces);
            let space_1 = available_spaces[space_1_i].clone();

            if space_1_i != num_spaces {
                available_spaces.swap(num_spaces - 1, space_1_i);
                num_spaces -= 1;
            }

            let space_2_i = rng.random_range(0..num_spaces);
            let space_2 = available_spaces[space_2_i].clone();

            //dbg!(space_1, space_2);

            debug_assert_ne!(space_1, space_2);
            debug_assert!(self.grid[space_1.0][space_1.1].is_some());
            debug_assert!(self.grid[space_2.0][space_2.1].is_some());

            self.flip_spaces(space_1.0, space_1.1, space_2.0, space_2.1);

            return Ok(());
        }

        pub fn flip_random_spaces(&mut self) {
            let mut rng = rand::rng();
            let block_x = rng.random_range(0..3);
            let block_y = rng.random_range(0..3);
            self.flip_random_spaces_in_block(block_x, block_y)
                .unwrap_or_else(|_| {
                    println!("Couldn't flip spaces in full block!");
                });
        }
    }

    impl Default for Solution {
        fn default() -> Self {
            Self {
                grid: [[None; BOARD_SIZE]; BOARD_SIZE],
            }
        }
    }

    impl Display for Solution {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let board = Board::from_solution(self.clone());
            return board.fmt(f);
        }
    }

    fn find_next_empty_num(taken: &mut [bool; 9]) -> i8 {
        for i in 0..9 {
            if !taken[i] {
                taken[i] = true;
                return (i + 1) as i8;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;
    use crate::Solution;
    #[test]
    fn correct_solution() {
        let board = Board::from_string(
            "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
        )
        .unwrap();
        let solution = Solution::from_board(Board::from_string(
            "000395060040082307310740028002050716080210549500467032954608000003570290020000000",
        ))
        .unwrap();

        let score = board.score_solution(&solution).unwrap();

        assert_eq!(score, 0);
    }

    #[test]
    fn incorrect_solution() {
        let board = Board::from_string(
            "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
        )
        .unwrap();
        let solution = Solution::from_board(Board::from_string(
            "000495060040082307310740028002050716080210549500467032954608000003570290020000000",
        ))
        .unwrap();

        let score = board.score_solution(&solution).unwrap();

        assert_eq!(score, 3);
    }

    #[test]
    fn errored_solution() {
        let board = Board::from_string(
            "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
        )
        .unwrap();
        let solution = Solution::from_board(Board::from_string(
            "000395060040082307310740028002050716080210549500467032954608000003570290020000003",
        ))
        .unwrap();

        let score = board.score_solution(&solution);

        assert!(score.is_err());
    }

    #[test]
    fn test_flip() {
        let mut solution = Solution::from_board(Board::from_string(
            "123456789123456789123456789123456789123456789123456789123456789123456789123456789",
        ));

        solution.flip_spaces(0, 0, 0, 1);
        assert_eq!(solution.grid[0][0], Some(2));
        assert_eq!(solution.grid[0][1], Some(1));
    }
}
