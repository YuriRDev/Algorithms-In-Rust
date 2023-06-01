use kmp::Kmp;

fn main() {
    let search = Kmp::new("ARARAQUARA", "ARARA ARARAQUARA");
    search.print_fail_transition();

    match search.search_first() {
        Ok((start, end)) => println!("Primeira ocorrencia encontrado entre [{},{}]", start, end),
        Err(e) => println!("{e}"),
    }
}
