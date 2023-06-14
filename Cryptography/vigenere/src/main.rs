use vigenere::Vigenere;

fn main() {
    let a = Vigenere::new("HELLOWORLD", "ONCOTO");
    println!("{}", a.crypt());

}




// 65-90