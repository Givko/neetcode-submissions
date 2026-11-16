use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut col_sudoku_map: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut cell_sudoku_map: HashMap<usize, HashSet<char>> = HashMap::new();

    for (i, row) in board.iter().enumerate() {
        let mut row_set: HashSet<char> = HashSet::new();
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
            let mut cell = (i /3) * 3 + (j/3);


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
