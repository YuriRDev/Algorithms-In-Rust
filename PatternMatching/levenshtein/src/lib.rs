use comfy_table::Table;

pub struct Levensthain {
    grid: Vec<Vec<i32>>,
    source: Vec<char>,
    target: Vec<char>,
    size: usize,
}

/// Constructor
impl Levensthain {
    /// Configure the Levenshtain stats.
    /// ### Here, you specify the source and target string 
    /// *Don't worry, it does **not** matter if you swap the two, the result is the same*
    /// ```
    /// let mut distance = Levensthain::new("I'm super cool", "I'm ultra cool");
    /// let value = distance.run();
    /// println!("Distance is: {value}");
    /// ```
    pub fn new(source: &str, target: &str) -> Levensthain {
        let size = if source.len() > target.len() {
            source.len()
        } else {
            target.len()
        };

        let (source_fill, target_fill) = fill_str(&source, &target);

        Levensthain {
            grid: Vec::new(),
            source: string_to_chars(&source_fill.to_string()),
            target: string_to_chars(&target_fill.to_string()),
            size,
        }
    }
}

/// Public
impl Levensthain {
    /// It's time to run the Levenshtain algorithm!
    /// Just easily type and wait for the return
    /// ```
    /// let mut distance = Levensthain::new(source, target);
    /// let value = distance.run();
    /// println!("Distance is: {value}");
    /// ```
    pub fn run(&mut self) -> i32 {
        self.start();
        self.fill_grid();

        self.get_distance_value()
    }

    // To-do
    pub fn pretty_print(&self) {
        // TO-DO, using comfy_table
    }
}

/// Private (Inside calculations)
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
        for count in 1..&self.size + 1 {
            self.get_pos_value((row, count));
        }
    }

    fn get_pos_value(&mut self, pos: (usize, usize)) {
        let (row, _) = pos;

        // Diagonal, Upper, Left;
        let possibilities = [
            self.check_upper(pos),
            self.check_left(pos),
            self.check_diagonal(pos),
        ];
        // I know, I know... I was lazy
        let answer = possibilities.iter().min().unwrap();

        self.grid[row].push(*answer);
    }

    fn get_distance_value(&self) -> i32 {
        self.grid[self.size][self.size]
    }
}

/// Current pos possibel values
impl Levensthain {
    /// Returns the upper cell *(in relation to (row, column)* value
    /// Literally just return ```grid[row-1][column]```
    fn check_upper(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = self.grid[row - 1][column];

        value + 1
    }

    /// Returns the left cell *(in relation to (row, column)* value
    ///  ```grid[row][column - 1]```
    fn check_left(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = &self.grid[row][column - 1];

        value + 1
    }

    /// Returns the diagonal (top, left) cell *(in relation to (row, column)* value
    ///  ```grid[row - 1][column - 1]```
    fn check_diagonal(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = self.grid[row - 1][column - 1];

        if self.equals_pos(pos) {
            return value;
        }
        value + 1
    }

    /// Verify if the char in the relation of row and column are the same
    /// If so, returns true,
    /// Else... **I know you can figure it out**
    fn equals_pos(&self, pos: (usize, usize)) -> bool {
        let (row, column) = pos;

        self.source[row - 1] == self.target[column - 1]
    }
}

/// Debug
impl Levensthain {
    /// Prints the biggest string size.
    /// *It's used to debug, idk why r u reading this
    pub fn print_biggest_size(&self) {
        println!("{}", &self.size);
    }

    /// Print's the grid value formated.
    /// In the future I plan to use **comfy_tables** to improve
    /// the visualization
    pub fn print_grid(&self) {
        println!("Printing grid!");
        for row in 0..self.grid.len() {
            println!("{:?}", &self.grid[row]);
        }
    }
}

/// Literally convers a string into a char
/// ```
/// let string_value = "Nice";
/// let chars_arr = string_to_chars(&string_value);
///
/// // chars_arr = ['N','i','c','e']
/// ```
/// Ngl, I don't know if rust has already this implemented
fn string_to_chars(value: &str) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();

    for ch in value.chars() {
        chars.push(ch);
    }

    chars
}

/// Sometimes between two strings, one is bigger then the other...
/// I know I could just used a more few if/elses to fix this, but I figured it
/// that this solution is just a little more expensive to process, but it makes the
/// program easier *(to write and read)*
fn fill_str(str1: &str, str2: &str) -> (String, String) {
    let mut str1_parsed = String::from(str1);
    let mut str2_parsed = String::from(str2);

    for cnt in str1.len()..str2.len() {
        let char_to_add = (str2.as_bytes()[cnt] + 1) as char;
        str1_parsed.push(char_to_add);
    }

    for cnt in str2.len()..str1.len() {
        let char_to_add = (str1.as_bytes()[cnt] + 1) as char;
        str2_parsed.push(char_to_add);
    }

    (str1_parsed, str2_parsed)
}
