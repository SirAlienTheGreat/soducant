use bit_vec::BitVec;
use std::fmt::{Display, Formatter};

const BOARD_SIZE: usize = 9;

#[derive(Debug, Clone)]
struct Board {
    grid: [[Option<i8>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn from_arr_with_zeros<T: Into<i8> + Clone>(arr: [[T; BOARD_SIZE]; BOARD_SIZE]) -> Self {
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

    fn from_string(str: &str) -> Self {
        assert!(
            str.len() == BOARD_SIZE * BOARD_SIZE,
            "Board construction from string requires properly sized board"
        );

        let mut arr: [[i8; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];

        for (i, c) in str.chars().enumerate() {
            assert!(
                c.is_ascii_digit(),
                "String entry contains a non-number {}",
                c
            );
            arr[i / BOARD_SIZE][i % BOARD_SIZE] = c.to_digit(10).unwrap() as i8;
        }

        return Self::from_arr_with_zeros(arr);
    }

    fn from_solution(board: Solution) -> Self {
        Self { grid: board.grid }
    }

    fn combine(&self, other: &Solution) -> Result<Board, Box<dyn std::error::Error>> {
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

    fn score_solution(&self, other: &Solution) -> Result<u32, Box<dyn std::error::Error>> {
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
                                combined.grid[block_i * 3 + i][block_j * 3 + j].unwrap_unchecked()
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
struct Solution {
    grid: [[Option<i8>; BOARD_SIZE]; BOARD_SIZE],
}

impl Solution {
    fn from_board(board: Board) -> Self {
        Self { grid: board.grid }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let board = Board::from_solution(self.clone());
        return board.fmt(f);
    }
}

fn main() {
    let board = Board::from_string(
        "278000401609100050005006900430809000706003000091000800000020173860001004107934685",
    );
    let solution = Solution::from_board(Board::from_string(
        "000495060040082307310740028002050716080210549500467032954608000003570290020000000",
    ));

    println!("Board:\n{}", board);
    println!("Solution:\n{}", solution);
    // 278000401609100050005006900430809000706003000091000800000020173860001004107934685

    let score = board.score_solution(&solution).unwrap();

    println!("{}", score);
}
