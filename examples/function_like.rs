fn main() {
    quanto::execute!(
        r#"
qubit[5] q1;
const uint SIZE = $size;
uint runtime_u = $u;
qubit[SIZE] q2;  // Declare a size-qubit register.

x q1[0];
z q2[SIZE - $delta];  // The index operand is of type `const uint`.


// Validity is implementation-defined.

x q1[runtime_u];
"#,
        size = 4u32,
        u = 2u32,
        delta = 2u32,
    )
    .unwrap();
}
