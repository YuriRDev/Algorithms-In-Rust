pub struct Levensthain {
    grid: Vec<Vec<i32>>,
    source: Vec<char>,
    target: Vec<char>,
    size: usize,
}

impl Levensthain {
    pub fn new(source: &str, target: &str) -> Levensthain {
        Levensthain {
            grid: Vec::new(),
            source: string_to_chars(&source),
            target: string_to_chars(&target),
            size: if source.len() > target.len() {
                source.len()
            } else {
                target.len()
            },
        }
    }
}

/// #####
/// Public
/// #####
impl Levensthain {
    pub fn run(&mut self) -> i32{
        self.start();
        self.fill_grid();

        self.get_distance_value()
    }
}
/// #####
/// Private (Inside calculations)
/// #####
impl Levensthain {
    fn start(&mut self) {
        let mut grid_tmp: Vec<Vec<i32>> = Vec::new();

        // Create first row
        let mut tmp: Vec<i32> = Vec::new();
        for value in 0..&self.size + 1 {
            tmp.push(value as i32);
        }
        grid_tmp.push(tmp);

        for value in 1..&self.size + 1 {
            grid_tmp.push(Vec::from([value as i32]));
        }

        self.grid = grid_tmp;
    }

    fn fill_grid(&mut self) {
        // Preencher fileira 1
        for row in 1..&self.size + 1 {
            self.fill_row(row);
        }
    }
    fn fill_row(&mut self, row: usize) {
        println!("Preenchendo fileira: {row}");

        for count in 1..&self.size + 1 {
            self.get_pos_value((row, count));
        }
    }

    fn get_pos_value(&mut self, pos: (usize, usize)) {
        let (row, column) = pos;

        // Diagonal, Upper, Left;
        let possibilities = [
            self.check_upper(pos),
            self.check_left(pos),
            self.check_diagonal(pos),
        ];
        let answer = possibilities.iter().min().unwrap();

        println!("Valor de {},{} tem que ser {}", row, column, answer);

        self.grid[row].push(*answer);
    }

    fn check_upper(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = self.grid[row - 1][column];
        println!("Upper de ({},{}) = {value}", row, column);

        value + 1
    }

    fn check_left(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = &self.grid[row][column - 1];
        println!("LEFT de ({},{}) = {value}", row, column);

        value + 1
    }

    fn check_diagonal(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = self.grid[row - 1][column - 1];
        println!("DIAGONAL DE de ({},{}) = {value}", row, column);

        if self.equals_pos(pos) {
            return value;
        }
        value + 1
    }

    fn equals_pos(&self, pos: (usize, usize)) -> bool {
        let (row, column) = pos;

        println!(
            "{} == {} ?",
            self.source[row - 1] as char,
            self.target[column - 1] as char
        );

        self.source[row - 1] == self.target[column - 1]
    }

    fn get_distance_value(&self) -> i32 {
        self.grid[self.size][self.size]
    }
}

/// #####
/// Debug (Inside calculations)
/// #####
impl Levensthain {
    pub fn print_biggest_size(&self) {
        println!("{}", &self.size);
    }

    pub fn print_grid(&self) {
        println!("Printing grid!");
        for row in 0..self.grid.len() {
            println!("{:?}", &self.grid[row]);
        }
    }
}

/// Convert a String type to Vec<char>
fn string_to_chars(value: &str) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();

    for ch in value.chars() {
        chars.push(ch);
    }

    chars
}