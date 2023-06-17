use vigenere::Vigenere;

fn main() {
    // let a = Vigenere::new("HELLOU", "ONCOTO");
    let a = Vigenere::new("VRNZHI", "ONCOTO");
    println!("{}", a.crypt());

}
