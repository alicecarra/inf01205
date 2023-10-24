pub(crate) use crate::header::Header;

#[derive(Clone, Debug)]
pub struct Aig {
    header: Header,
    child_0: Vec<usize>,
    child_1: Vec<usize>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

impl Aig {
    pub fn new(file_path: &std::path::PathBuf) -> Self {
        let content = std::fs::read_to_string(file_path).expect("Error reading file");
        let mut lines = content.lines();
        let mut header = Header::default();
        header.process_header(lines.next().expect("File empty?").into());

        println!("{header:?}");

        let mut aig = Aig {
            header: header.clone(),
            child_0: vec![0; header.max_variables + 1],
            child_1: vec![0; header.max_variables + 1],
            inputs: vec![],
            outputs: vec![],
        };

        let _ = lines
            .enumerate()
            .map(|(count, content)| {
                aig.process_line(content, count);
            })
            .collect::<()>();

        println!("{aig:?}");

        aig
    }

    fn process_line(&mut self, content: &str, count: usize) {
        let content = content.to_string();
        let line_number = count + 1;

        let number_of_inputs = self.header.number_of_inputs;
        let number_of_outputs = self.header.number_of_outputs;
        let number_of_ands = self.header.number_of_ands;

        if line_number <= number_of_inputs {
            self.process_input_line(content.to_owned())
        } else if line_number <= number_of_inputs + number_of_outputs {
            self.process_output_line(content.to_owned())
        } else if line_number <= number_of_inputs + number_of_outputs + number_of_ands {
            self.process_and_line(content.to_owned())
        }
    }

    fn process_input_line(&mut self, line: String) {
        println!("in={line}");
        let input = line.parse::<usize>().expect("Error parsing input line");
        self.inputs.push(input)
    }

    fn process_output_line(&mut self, line: String) {
        println!("out={line}");
        let output = line.parse::<usize>().expect("Error parsing output line");
        self.outputs.push(output)
    }

    fn process_and_line(&mut self, line: String) {
        let mut splitter = line.split(' ');
        let output = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        println!("connect pin={output}");

        let input_0 = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        println!("connect pin={input_0}");

        let input_1 = splitter
            .next()
            .expect("Incomplete and line")
            .parse::<usize>()
            .expect("Error parsing and line");
        println!("connect pin={input_1}");

        let index = output / 2;

        self.child_0.insert(index, input_0);
        self.child_1.insert(index, input_1);
    }

    pub fn print(&self) {
        let aig = self.clone();
        for input in aig.inputs {
            println!("input = {input}");
        }

        for output in aig.outputs {
            println!("output = {output}");
        }

        for and in aig.header.number_of_inputs + 1
            ..aig.header.number_of_inputs + aig.header.number_of_ands + 1
        {
            println!(
                "AND {}={}*{}",
                and * 2,
                aig.child_0.get(and).unwrap(),
                aig.child_1.get(and).unwrap()
            )
        }
    }

    pub fn delay(&self) {
        let input_delay = 0;
        let and_delay = 2;
        let inversor_delay = 1;

        let aig = self.clone();
        let max_variables = aig.header.max_variables;
        let mut delays = vec![0; max_variables];

        for input in aig.inputs {
            delays.insert(input, 0);
            println!("input = {input} - delay = {input_delay}");
        }

        for and in aig.header.number_of_inputs + 1
            ..aig.header.number_of_inputs + aig.header.number_of_ands + 1
        {
            let child_0 = aig.child_0.get(and).unwrap();
            let child_1 = aig.child_1.get(and).unwrap();

            let child_0_delay = delays.get(child_0 / 2).unwrap() + ((child_0 % 2) * inversor_delay);
            let child_1_delay = delays.get(child_1 / 2).unwrap() + ((child_1 % 2) * inversor_delay);

            let delay = std::cmp::max(child_0_delay, child_1_delay) + and_delay;

            delays.insert(and, delay);
            println!("AND {}={}*{} - delay = {delay}", and * 2, child_0, child_1)
        }

        for output in aig.outputs {
            let delay = delays.get(output / 2).unwrap() + ((output % 2) * inversor_delay);

            delays.insert(output, delay);

            println!("output = {output} - delay = {delay}");
        }

        println!("{delays:?}");

        let max_delay = delays.into_iter().fold(0, std::cmp::max);

        println!("Max delay: {max_delay}");
    }
}
