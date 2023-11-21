#[derive(Default, Clone, Debug)]
pub struct Header {
    pub max_variables: usize,
    pub number_of_inputs: usize,
    pub number_of_latches: usize,
    pub number_of_outputs: usize,
    pub number_of_ands: usize,
}

impl Header {
    pub fn process_header(&mut self, line: String) {
        let mut splitter = line.split(' ');

        let _discarded_aag = splitter.next().expect("Incomplete and line");

        let max_variables = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        self.max_variables = max_variables;
        // println!("connect pin={max_variables}");

        let number_of_inputs = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        self.number_of_inputs = number_of_inputs;
        // println!("connect pin={number_of_inputs}");

        let number_of_latches = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        self.number_of_latches = number_of_latches;

        let number_of_outputs = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        self.number_of_outputs = number_of_outputs;
        // println!("connect pin={number_of_outputs}");

        let number_of_ands = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        self.number_of_ands = number_of_ands;
        // println!("connect pin={number_of_ands}");
    }

    pub fn generate_verilog_module_header(&self) -> String {
        let mut interface = String::new();

        for input in 1..=self.number_of_inputs {
            interface.push_str(format!("n{}, ", input * 2).as_str())
        }

        for output in 1..=self.number_of_outputs {
            interface.push_str(format!("n{}o, ", output * 2).as_str())
        }

        // Remove last ", "
        interface.pop();
        interface.pop();

        interface
    }
}
