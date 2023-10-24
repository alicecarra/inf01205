use cad::aig::Aig;

fn main() {
    let aig = Aig::new(&std::path::PathBuf::from("./sat".to_string()));
    aig.delay();
}
