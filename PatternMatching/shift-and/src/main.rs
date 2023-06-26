use shift_and::ShiftAnd;
fn main() {
    let mut shiftand = ShiftAnd::new("teste", "os testeste");
    let patterns_window = shiftand.search();
    
    // Wow! You can print the bit mask!
    shiftand.print_bit_mask();

    assert_eq!(patterns_window, [(3, 7), (6, 10)]);
}
