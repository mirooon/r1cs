use std::collections::HashMap;
use crate::r1cs::R1CS;

pub struct ConstraintBuilder {
    pub constraints: Vec<(Vec<f64>, Vec<f64>, Vec<f64>)>,
    pub variables: HashMap<String, usize>,
    next_var_index: usize,
}

impl ConstraintBuilder {
    pub fn new() -> Self {
        ConstraintBuilder {
            constraints: Vec::new(),
            variables: HashMap::new(),
            next_var_index: 0,
        }
    }

    pub fn add_variable(&mut self, name: &str) -> usize {
        if let Some(&index) = self.variables.get(name) {
            index
        } else {
            let index = self.next_var_index;
            self.variables.insert(name.to_string(), index);
            self.next_var_index += 1;
            index
        }
    }

    pub fn add_multiply(&mut self, x: usize, y: usize, output_name: &str) -> usize {
        let output_index = self.add_variable(output_name);

        let mut a = vec![0.0; self.next_var_index];
        let mut b = vec![0.0; self.next_var_index];
        let mut c = vec![0.0; self.next_var_index];

        a[x] = 1.0;
        b[y] = 1.0;
        c[output_index] = -1.0;

        self.constraints.push((a, b, c));
        output_index
    }

    pub fn add_addition(&mut self, x: usize, y: usize, output_name: &str) -> usize {
        let output_index = self.add_variable(output_name);

        let mut a = vec![0.0; self.next_var_index];
        let mut b = vec![0.0; self.next_var_index];
        let mut c = vec![0.0; self.next_var_index];

        a[x] = 1.0;
        b[0] = 1.0;
        c[output_index] = -1.0;

        self.constraints.push((a, b, c));
        output_index
    }

    pub fn add_addition_constant(&mut self, left: usize, constant: f64, result_name: &str) -> usize {
        // Add the result variable
        let result = self.add_variable(result_name);
    
        // Create the constraint vectors
        let mut a = vec![0.0; self.variables.len()];
        let mut b = vec![0.0; self.variables.len()];
        let mut c = vec![0.0; self.variables.len()];
    
        // Populate the A, B, and C vectors for this constraint
        a[left] = 1.0; // Left variable contributes to A
        b[0] = 1.0; // B uses the ~one variable (constant multiplier is applied here)
        c[result] = 1.0; // Result variable contributes to C
    
        // Add the constant contribution directly
        a[0] = constant;
    
        // Add the constraint
        self.constraints.push((a, b, c));
    
        result
    }

    pub fn build_r1cs(self) -> R1CS {
        R1CS {
            constraints: self.constraints,
            variables: self
                .variables
                .iter()
                .map(|(var, _)| var.clone())
                .collect(),
        }
    }
}
