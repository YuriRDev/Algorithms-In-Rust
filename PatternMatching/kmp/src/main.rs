use std::collections::HashMap;

use kmp::Kmp;

fn main() {

    enum a = {

    }
    let mut foo: HashMap<&str, i32> = HashMap::new();
    foo.insert("=", 1);
    foo.insert("+", 2);
    foo.insert("-", 2);

    let boo_value = match foo.get("invalid_key") {
        Some(&e) => e,
        _ => 0,
    };


    // let search = Kmp::new("ARARAQUARA", "ARARA ARARAQUARA");
    // search.print_fail_transition();

    // match search.search_first() {
    //     Ok((start, end)) => println!("Primeira ocorrencia encontrado entre [{},{}]", start, end),
    //     Err(e) => println!("{e}"),
    // }
}
