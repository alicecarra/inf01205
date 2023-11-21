pub mod utils;

use cad::aig::Aig;
use utils::get_max_delay;

fn main() {
    let aig = Aig::new(&std::path::PathBuf::from("./sat".to_string()));
    aig.print();

    let input_delay = 0;
    let and_delay = 2;
    let inversor_delay = 1;

    let delays = aig.generate_delays(input_delay, inversor_delay, and_delay);
    let max_delay = get_max_delay(delays);
    println!("Max delay: {max_delay}");

    let verilog = aig.generate_verilog_module("Aig".to_string());

    println!("{verilog}")
}
