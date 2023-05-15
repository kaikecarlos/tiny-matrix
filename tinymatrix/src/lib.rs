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

    pub fn concat_rows(&mut self, row1: usize, row2: usize) {
        assert!(row1 < self.rows && row2 < self.rows, "Invalid row indices!");
        assert!(
            self.cols > 1,
            "Matrix needs at least two columns to be concatenated!"
        );
        let mut new_row = Vec::with_capacity(self.cols);

        let r1_start = row1 * self.cols;
        let r1_end = row1 + self.cols;

        new_row.extend_from_slice(&self.data[r1_start..r1_end]);

        let r2_start = row2 * self.cols;
        let r2_end = row2 + self.cols;

        new_row.extend_from_slice(&self.data[r2_start..r2_end]);

        let insert_index = row1.min(row2);
        let insert_start = insert_index * self.cols;
        // self.data.insert(insert_start, new_row);
        self.rows += 1;
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
