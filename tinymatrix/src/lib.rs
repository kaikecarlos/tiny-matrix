#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn from_vector(rows: usize, cols: usize, values: Vec<f64>) -> Self {
        if rows * cols != values.len() {
            panic!("The vector input needs to match with the amount of columns and rows!");
        } else {
            Matrix {
                rows,
                cols,
                data: values,
            }
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn is_squared(&self) -> bool {
        if self.cols == self.rows {
            return true;
        } else {
            return false;
        }
    }

    pub fn main_diagonal(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        if self.is_squared() {
            let mut m_diagonal: Vec<f64> = Vec::new();
            let mut a_diagonal: Vec<f64> = Vec::new();
            let mut b_diagonal: Vec<f64> = Vec::new();

            for i in 0..self.rows {
                for j in 0..self.cols {
                    if i == j {
                        m_diagonal.push(self.get(i, i));
                    } else if i > j {
                        b_diagonal.push(self.get(i, j));
                    } else {
                        a_diagonal.push(self.get(i, j));
                    }
                }
            }
            (m_diagonal, a_diagonal, b_diagonal)
        } else {
            panic!("The matrix needs to be squared!")
        }
    }

    pub fn is_u_triangular(&self) -> bool {
        if !self.is_squared() {
            return false;
        } else {
            let (_, _, b_diagonal) = self.main_diagonal();
            let mut z = 0;
            for i in 0..b_diagonal.len() {
                if b_diagonal[i] == 0.0 {
                    z += 1;
                }
            }
            if z == b_diagonal.len() {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn is_l_triangular(&self) -> bool {
        if !self.is_squared() {
            return false;
        } else {
            let (_, a_diagonal, _) = self.main_diagonal();
            let mut z = 0;
            for i in 0..a_diagonal.len() {
                if a_diagonal[i] == 0.0 {
                    z += 1;
                }
            }
            if z == a_diagonal.len() {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn concat_rows(&self, other: &Self) -> Self {
        if self.rows != other.rows {
            panic!("Both matrices need to have the same amount of rows!");
        } else {
            let mut result: Vec<f64> = Vec::with_capacity(self.data.len() + other.data.len());

            for i in 0..self.rows {
                let s_index1 = i * self.cols;
                let e_index1 = s_index1 + self.cols;
                result.extend_from_slice(&self.data[s_index1..e_index1]);

                let s_index2 = i * other.cols;
                let e_index2 = s_index2 + other.cols;
                result.extend_from_slice(&other.data[s_index2..e_index2]);
            }

            Matrix {
                rows: self.rows,
                cols: self.cols * 2,
                data: result,
            }
        }
    }

    pub fn concat_cols(&self, other: &Self) -> Self {
        if self.cols != other.cols {
            panic!("Both matrices needs to have the same amount of columns!");
        } else {
            let mut result: Vec<f64> = Vec::with_capacity(self.data.len() + other.data.len());

            for i in 0..self.rows {
                for j in 0..self.cols {
                    result.push(self.data[i * self.cols + j]);
                }

                for j in 0..other.cols {
                    result.push(other.data[i * other.cols + j]);
                }
            }

            Matrix {
                rows: self.rows,
                cols: self.cols + other.cols,
                data: result,
            }
        }
    }

    pub fn transpose(&self) -> Self {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }

    pub fn identity(&self) -> Self {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            result.data[i * self.rows + i] = 1.0;
        }
        result
    }

    pub fn determinant(&self) -> Self {
        unimplemented!("The determinant of a matrix");
    }

    pub fn lu_decomposition(&self) -> Self {
        unimplemented!("The LU decomposition of a matrix");   
    }
    pub fn print_matrix(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{:.2} ", self.get(i, j))
            }
            println!();
        }
        println!();
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Self::Output {
        if self.cols != other.rows {
            panic!(
                "The amount of columns needs to be equal to the amount of rows for multiplication!"
            )
        } else {
            let mut result = Matrix::new(self.rows, other.cols);
            for i in 0..self.rows {
                for j in 0..other.cols {
                    let mut sum = 0.0;
                    for k in 0..self.cols {
                        sum += self.get(i, k) * other.get(k, j);
                    }
                    result.set(i, j, sum);
                }
            }
            result
        }
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Self::Output {
        if self.cols != other.rows {
            panic!(
                "The amount of columns needs to be equal to the amount of rows for multiplication!"
            )
        } else {
            Matrix {
                rows: self.rows,
                cols: self.cols,
                data: self
                    .data
                    .iter()
                    .zip(other.data.iter())
                    .map(|(&a, &b)| a + b)
                    .collect(),
            }
        }
    }
}

impl std::ops::Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Self::Output {
        if self.cols != other.rows {
            panic!(
                "The amount of columns needs to be equal to the amount of rows for multiplication!"
            )
        } else {
            Matrix {
                rows: self.rows,
                cols: self.cols,
                data: self
                    .data
                    .iter()
                    .zip(other.data.iter())
                    .map(|(&a, &b)| a - b)
                    .collect(),
            }
        }
    }
}
