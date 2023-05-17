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



    let (main_diagonal, a_diagonal, b_diagonal) = Matrix::main_diagonal(&m1);
    println!("{:?}", main_diagonal);
    println!("{:?}", a_diagonal);
    println!("{:?}", b_diagonal);

    // Example of an upper triangular matrix (all elements below the main diagonal are zero)
    let u_v = vec![
        1.0, 2.0, 3.0,
        0.0, -1.0, 10.0,
        0.0, 0.0, 0.0
    ];
    let u_m = Matrix::from_vector(3, 3, u_v);
    println!("{:?}", u_m.is_u_triangular());


    // Test joining cols and joining rows
    let v_test1 = vec![
        1.0, 2.0,
        4.0, 5.0,
    ];

    let v_test2 = vec![
        5.0, 6.0,
        8.0, 9.0,
    ];

    let cm_test1 = Matrix::from_vector(2, 2, v_test1);
    let cm_test2 = Matrix::from_vector(2, 2, v_test2);

    let result_sum = cm_test1.clone() + cm_test2.clone();
    result_sum.print_matrix();

    let result_sub = cm_test1.clone() - cm_test2.clone();
    result_sub.print_matrix();

    let result_mult = cm_test1.clone() * cm_test2.clone();
    result_mult.print_matrix();

    let result_id = cm_test1.clone().identity();
    result_id.print_matrix();

    let cm_new = cm_test1.concat_cols(&cm_test2);
    cm_new.print_matrix();

}