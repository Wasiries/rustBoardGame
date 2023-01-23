pub struct Board {
    table: Vec<Vec<char>>,
}


impl Board {
    pub fn create_board() -> Board {
        let mut result: Vec<Vec<char>> = vec![];
        for rowCounter in 0..4 {
            let mut row: Vec<char> = vec![];
            for columnCounter in 0..4 {
                let mut column: char = char::from('#');
                row.push(column);
            }
            result.push(row);
        }
        let mut output = Board { table: result };
        return output;
    }


    pub fn print_board(&self) {
        for row in self.table {
            println!("+---+---+---+");
            for column in row {
                print!("| {} ", column);
                println!("|");
            }
        } 
        println!("+---+---+---+");
    }


    pub fn set_value(&self, letter: char, row: usize, column: usize) -> Board {
        let mut table = self.table;
        table[row][column] = letter;
        return Board{table: table};
    }
}