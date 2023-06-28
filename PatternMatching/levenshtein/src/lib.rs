pub struct Levensthain<'s> {
    grid: Vec<Vec<i32>>,
    source: &'s str,
    target: &'s str,
}

impl<'s> Levensthain<'s> {
    pub fn new<'a>(source: &'a str, target: &'a str) -> Levensthain<'a> {
        Levensthain {
            grid: Vec::new(),
            source,
            target,
        }
    }
}

/// Public methods
impl<'s> Levensthain<'s> {
    pub fn run(&mut self) -> i32 {
        self.start();
        self.fill_grid();

        self.get_distance_value()
    }

    pub fn print_grid(&self) {
        for row in 0..self.grid.len() {
            println!("{:?}", &self.grid[row]);
        }
    }
}

/// Private
impl<'s> Levensthain<'s> {
    fn start(&mut self) {
        let mut grid_tmp: Vec<Vec<i32>> = Vec::new();

        // Create first row
        let mut tmp: Vec<i32> = Vec::new();
        for value in 0..&self.source.len() + 1 {
            tmp.push(value as i32);
        }
        grid_tmp.push(tmp);

        // preencher colunas
        for value in 1..&self.target.len() + 1 {
            grid_tmp.push(Vec::from([value as i32]));
        }

        self.grid = grid_tmp;
    }

    fn fill_grid(&mut self) {
        // Preencher fileira 1
        for row in 1..&self.target.len() + 1 {
            self.fill_row(row);
        }
    }

    fn fill_row(&mut self, row: usize) {
        for count in 1..&self.source.len() + 1 {
            self.get_pos_value((row, count));
        }
    }

    fn get_pos_value(&mut self, pos: (usize, usize)) {
        let (row, _) = pos;

        let upper = self.check_upper(pos);
        let left = self.check_left(pos);
        let diagonal = self.check_diagonal(pos);

        let mut answer = upper;

        if left < answer {
            answer = left;
        }
        if diagonal < answer {
            answer = diagonal;
        }

        self.grid[row].push(answer);
    }

    fn get_distance_value(&self) -> i32 {
        self.grid[self.target.len()][self.source.len()]
    }
}

/// Border-Grid checker
impl<'s> Levensthain<'s> {
    fn check_upper(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = self.grid[row - 1][column];

        value + 1
    }

    fn check_left(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = &self.grid[row][column - 1];

        value + 1
    }

    fn check_diagonal(&self, pos: (usize, usize)) -> i32 {
        let (row, column) = pos;

        let value = self.grid[row - 1][column - 1];

        if self.equals_pos(pos) {
            return value;
        }
        value + 1
    }

    fn equals_pos(&self, pos: (usize, usize)) -> bool {
        let (row, column) = pos;

        let src_len = self.source.as_bytes().len() - 1;
        let src_mus = column - 1;

        let tgt_len = self.target.as_bytes().len() - 1;
        let tgt_mus = row - 1;

        if src_mus > src_len || tgt_mus > tgt_len {
            false
        } else {
            self.source.as_bytes()[column - 1] == self.target.as_bytes()[row - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_chars() {
        let mut lev = Levensthain::new("abcd", "bbcd");
        assert_eq!(lev.run(), 1);
    }

    #[test]
    fn eight_chars() {
        let mut lev = Levensthain::new("palekd93", "alekd92");
        assert_eq!(lev.run(), 2);
    }
    
    #[test]
    fn sixteen_chars() {
        let mut lev = Levensthain::new("ak39818@91028172", "ak2910@");
        assert_eq!(lev.run(), 11);
    }
    
    #[test]
    fn thirtytwo_chars() {
        let mut lev = Levensthain::new("9ALDKSJAM.,;A92J#$%1938261KAICOA", "9ALDKSJAMKKLAMCN#$%1@@DLAKSAICOA");
        assert_eq!(lev.run(), 13);
    }

    #[test]
    fn sixtyfour_chars() {
        let mut lev = Levensthain::new("Nam quis nulla. Integer malesuada. In in enim a arcu imperdiet m", "Nam quis nulla. Nam quis n malesuada enim a arcu imperdiet m");
        assert_eq!(lev.run(), 17);
    }
}
