use lz77::Lz77;

fn main() {
    let comp = Lz77::new("ABAABAABAC", 3);
    comp.start();
}
