use kmp::Kmp;
fn main() {
    let search = Kmp::new("araraquara", "aranha arranha a araraquara");
    search.print_fail_transition();
}
