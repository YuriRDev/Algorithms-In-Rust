use FLPM::Flmp;

fn main() {
    let a = Flmp::new("Aranha arranha a arra d aa", "arra").search();
    println!("{:?}", a);
}
