#![crate_name = "doc"]

struct BitMasksSudoKu {}

impl BitMasksSudoKu {
    /// Create 3 arrays of size N (one for rows, columns, and boxes)
    /// The boxes are indexed 0 to 8. (in order to find the box index of an element: int(row/3*3+col/3)
    /// Map the initial values of the grid first
    /// Each time add/remove an element to/from the grid set the bit to 1/0 to corresponding bitmasks
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row = vec![];
        let mut col = vec![];
        let mut boxz = vec![];

        let i = 0;
        let j = 0;
        let nr: u8 = 2;

        board[i][j] = nr as char;
        row[i] |= 1 << nr;
        col[j] |= 1 << nr;
        boxz[0] |= 1 << nr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let mut input: Vec<Vec<char>> = vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        BitMasksSudoKu::solve_sudoku(&mut input);
    }
}

struct CrossHatchingSudoku {}

impl CrossHatchingSudoku {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {}
}