use std::vec::Vec;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Grid {
    pub array: Vec<u8>,
    size:  usize,
}

impl Grid {

    fn new(grid_as_str: &str, sudoku_size: usize) -> Grid {

        let length = sudoku_size * sudoku_size * sudoku_size * sudoku_size;
        if grid_as_str.len() != length {
            panic!("Invalid input")
        }
        let mut vec = vec![0; length];


        for (i, c) in grid_as_str.chars().enumerate() {
            let char_as_int = c.to_digit(32);
            match char_as_int {
                Some(x) =>  vec[i] = x as u8,
                None => panic!("Invalid input")
            }
        }
        Grid {array:vec, size:length}
    }

    fn check(&self) -> bool {
        let squared = self.size*self.size;

        let mut used_row_values = vec![false; squared];
        let mut used_col_values = HashMap::new();
        for i in 0..squared {
            used_col_values.insert(i, vec![false; squared]);
        }
        let mut used_box_values = HashMap::new();
        for i in 0..self.size-1 {
            for j in 0..self.size-1 {
                used_box_values.insert((i,j), vec![false; squared]);
            }
        }

        for (i, v) in self.array.iter().enumerate() {
            if used_row_values[(*v as usize)-1] {
                return false;
            }
            used_row_values[(*v as usize)-1] = true;

            let col_index = i % squared;

            if col_index == 0 {
                for v in &mut used_row_values {
                    *v = false;
                }
            }

            match used_col_values.get(&col_index) {
                Some(s) => if s[(*v as usize)-1] { return false;} else {s[(*v as usize)-1] = true;},
                None => panic!("Kaboom")
            }
        }
        return true;
    }

    fn solve(&self) -> Grid {
        let g = Grid {array:self.array, size:self.size};
        g
    }
}

#[cfg(test)]
mod sudoku_test {

    use super::*;

    #[test]
    fn init_sudoku() {
        let test_grid_9x9: [&str; 7] = [
            "004300209005009001070060043006002087190007400050083000600000105003508690042910300",
            "040100050107003960520008000000000017000906800803050620090060543600080700250097100",
            "600120384008459072000006005000264030070080006940003000310000050089700000502000190",
            "497200000100400005000016098620300040300900000001072600002005870000600004530097061",
            "005910308009403060027500100030000201000820007006007004000080000640150700890000420",
            "100005007380900000600000480820001075040760020069002001005039004000020100000046352",
            "009065430007000800600108020003090002501403960804000100030509007056080000070240090"
        ];

        for grid_data in &test_grid_9x9 {
            let g = Grid::new(grid_data, 3);
        }
    }
}