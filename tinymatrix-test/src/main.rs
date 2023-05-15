use tinymatrix::Matrix;


fn main() {
    let vt1 = vec![
        1.0, 2.0, 3.0,
        -2.0, -1.0, 10.0,
        5.0, 6.0, -1.5
    ];
    let vt2 = vec![
        4.0, 1.0, 0.0, 
        -2.0, -5.0, 1.0, 
        -1.0, 3.2, 0.0
    ];
    let m1 = Matrix::from_vector(3, 3, vt1);
    m1.print_matrix();
    let m2 = Matrix::from_vector(3, 3, vt2);
    m2.print_matrix();

    let result = m1.multiply(&m2);
    result.print_matrix();

    let main_diagonal = Matrix::main_diagonal(&m1);
    print!("{:?}", main_diagonal);
}