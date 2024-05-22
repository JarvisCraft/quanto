fn main() {
    quanto::execute!(
        r#"
qubit[5] q1;
const uint SIZE = 4;
uint runtime_u = 2;
qubit[SIZE] q2;  // Declare a 4-qubit register.

x q1[0];
z q2[SIZE - 2];  // The index operand is of type `const uint`.


// Validity is implementation-defined.

x q1[runtime_u];
"#,
        1,
        2,
        3,
    )
    .unwrap();
}
