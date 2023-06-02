use lz77::Lz77;

fn main() {
    let comp = Lz77::new("ABABABCACACB", 3);
    comp.start();
}
