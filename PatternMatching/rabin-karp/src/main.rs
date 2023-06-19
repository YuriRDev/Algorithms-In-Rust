use rabin_karp::RabinKarp;

fn main() {
    let a = RabinKarp::new("jkl", "abcdefghijklmnop");
    a.print();
}
