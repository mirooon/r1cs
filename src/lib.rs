pub mod constraints;
pub mod variables;
pub mod r1cs;

pub use constraints::ConstraintBuilder;
pub use variables::VariableManager;
pub use r1cs::R1CS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_variable() {
        let mut builder = ConstraintBuilder::new();
        let x = builder.add_variable("x");
        let y = builder.add_variable("y");
        let z = builder.add_variable("z");

        assert_eq!(x, 0);
        assert_eq!(y, 1);
        assert_eq!(z, 2);
        assert_eq!(builder.variables.len(), 3);
    }

    #[test]
    fn test_add_multiply_constraint() {
        let mut builder = ConstraintBuilder::new();
        let x = builder.add_variable("x");
        let y = builder.add_variable("y");
        let z = builder.add_multiply(x, y, "z");

        assert_eq!(z, 2);
        assert_eq!(builder.constraints.len(), 1);

        let (a, b, c) = &builder.constraints[0];
        assert_eq!(a[x], 1.0);
        assert_eq!(b[y], 1.0);
        assert_eq!(c[z], -1.0);
    }

    #[test]
    fn test_add_addition_constraint() {
        let mut builder = ConstraintBuilder::new();
        let x = builder.add_variable("x");
        let y = builder.add_variable("y");
        let w = builder.add_addition(x, y, "w");

        assert_eq!(w, 2);
        assert_eq!(builder.constraints.len(), 1);

        let (a, b, c) = &builder.constraints[0];
        assert_eq!(a[x], 1.0);
        assert_eq!(b[0], 1.0);
        assert_eq!(c[w], -1.0);
    }

    #[test]
    fn test_build_r1cs() {
        let mut builder = ConstraintBuilder::new();
        let x = builder.add_variable("x");
        let y = builder.add_variable("y");
        let z = builder.add_multiply(x, y, "z");
        let w = builder.add_addition(z, x, "w");

        let r1cs = builder.build_r1cs();

        assert_eq!(r1cs.constraints.len(), 2);
        assert_eq!(r1cs.variables.len(), 4);
        assert!(r1cs.variables.contains(&"x".to_string()));
        assert!(r1cs.variables.contains(&"y".to_string()));
        assert!(r1cs.variables.contains(&"z".to_string()));
        assert!(r1cs.variables.contains(&"w".to_string()));
    }
    #[test]
    fn test_polynomial_y_equals_x3_plus_x_plus_5() {
        let mut builder = ConstraintBuilder::new();
    
        let x = builder.add_variable("x");
        let x2 = builder.add_multiply(x, x, "x^2");
        let x3 = builder.add_multiply(x2, x, "x^3");
        let x3_plus_x = builder.add_addition(x3, x, "x^3 + x");
        builder.add_addition_constant(x3_plus_x, 5.0, "y");
    
        let r1cs = builder.build_r1cs();
        display_r1cs(&r1cs);
    }
}

fn display_r1cs(r1cs: &R1CS) {
    println!("Variables: {:?}", r1cs.variables);

    for (i, (a, b, c)) in r1cs.constraints.iter().enumerate() {
        println!(
            "Constraint {}: A = {:?}, B = {:?}, C = {:?}",
            i, a, b, c
        );
    }
}
