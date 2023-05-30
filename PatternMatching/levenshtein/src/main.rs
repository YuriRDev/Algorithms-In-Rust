fn main() {
    const main_string: &[char] = &['b', 'e', 'n', 'y', 'a', 'm'];
    const other_string: &[char] = &['e', 'p', 'h', 'r', 'e', 'm'];

    let biggest_str: usize = if main_string.len() > other_string.len() {
        main_string.len()
    } else {
        other_string.len()
    };

    let mut grid = start_grid(biggest_str);
}

fn start_grid(size: usize) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    // Create first row
    let mut tmp: Vec<i32> = Vec::new();
    for value in 0..size + 1 {
        tmp.push(value as i32);
    }
    grid.push(tmp);

    for value in 1..size + 1 {
        grid.push(Vec::from([value as i32]));
    }

    grid
}
