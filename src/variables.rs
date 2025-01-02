use std::collections::HashMap;

pub struct VariableManager {
    pub variables: HashMap<String, usize>,
    pub next_index: usize,
}

impl VariableManager {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        variables.insert("1".to_string(), 0);
        Self {
            variables,
            next_index: 1,
        }
    }

    pub fn add_variable(&mut self, name: &str) -> usize {
        if let Some(&index) = self.variables.get(name) {
            return index;
        }
        let index = self.next_index;
        self.variables.insert(name.to_string(), index);
        self.next_index += 1;
        index
    }

    pub fn get_variables(&self) -> Vec<String> {
        let mut vars = vec!["1".to_string()];
        vars.extend(
            self.variables
                .iter()
                .filter(|&(k, _)| k != "1")
                .map(|(k, _)| k.clone()),
        );
        vars
    }
}
