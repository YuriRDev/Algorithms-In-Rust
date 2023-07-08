use FLPM::Flpm;

fn main() {
    let flpm = Flpm::new("can you can a can as a canner can can a can?", "can");
    let result = flpm.search();
    
    println!("{:?}", result);
}
