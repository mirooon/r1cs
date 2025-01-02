pub struct R1CS {
    pub constraints: Vec<(Vec<f64>, Vec<f64>, Vec<f64>)>,
    pub variables: Vec<String>,
}

impl R1CS {
    pub fn display(&self) {
        println!("R1CS Constraints:");
        for (i, (a, b, c)) in self.constraints.iter().enumerate() {
            println!("Constraint {}: A = {:?}, B = {:?}, C = {:?}", i, a, b, c);
        }

        println!("\nVariables:");
        for (i, var) in self.variables.iter().enumerate() {
            println!("{}: {}", i, var);
        }
    }
}
