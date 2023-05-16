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

    pub fn main_diagonal(&self) -> Vec<f64> {
        if self.is_squared() {
            let mut diagonal = vec![0.0; self.rows];
            for i in 0..self.rows {
                diagonal[i] = self.get(i, i);
            }
            diagonal
        } else {
            panic!("The matrix needs to be squared!")
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
                data: result
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
                    result.push(self.data[i * self.cols +j]);
                }

                for j in 0..other.cols {
                    result.push(other.data[i * other.cols + j]);
                }
            }

            Matrix {
                rows: self.rows,
                cols: self.cols + other.cols,
                data: result
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

    pub fn multiply(&self, other: &Self) -> Self {
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
