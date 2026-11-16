use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut col_sudoku_map: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut cell_sudoku_map: HashMap<usize, HashSet<char>> = HashMap::new();

    for (i, row) in board.iter().enumerate() {
        let mut row_set: HashSet<char> = HashSet::new();
        let mut cell_row = 0;
        if i <= 2 {
            cell_row = 0;
        }
        if i >= 3 && i <= 5 {
            cell_row = 1;
        }
        if i >= 6 && i <= 8 {
            cell_row = 2;
        }

        for (j, row_item) in row.iter().enumerate() {
            if !row_item.is_digit(10) {
                continue;
            }

            //Check for invalid row
            if row_set.contains(row_item) {
                return false;
            }

            row_set.insert(*row_item);

            //Check for invalid column
            let col_set = col_sudoku_map.entry(j).or_insert(HashSet::new());
            if col_set.contains(row_item) {
                return false;
            }
            col_set.insert(*row_item);

            //Check for invalid cell
            let mut cell_col = 0;
            if j <= 2 {
                cell_col = 0;
            }
            if j >= 3 && j <= 5 {
                cell_col = 1;
            }
            if j >= 6 && j <= 8 {
                cell_col = 2;
            }
            let mut cell = 0;
            if cell_row == 0 {
                cell = cell_col;
            }
            if cell_row > 0 {
                cell = cell_col + cell_row * 3;
            }


            let cell_set = cell_sudoku_map.entry(cell).or_insert(HashSet::new());
            if cell_set.contains(row_item) {
                return false;
            }
            cell_set.insert(*row_item);
        }
    }

    return true;
    }
}
