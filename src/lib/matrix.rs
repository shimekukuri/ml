use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    clone, f64,
    fmt::{Debug, Formatter, Result},
    sync::mpsc,
};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

pub struct Matrix_2 {
    pub data: Vec<f64>,
    pub rows_size: usize,
}

impl Matrix_2 {
    pub fn zeros(rows: usize, cols: usize) -> Matrix_2 {
        Matrix_2 {
            data: vec![0.0; rows * cols],
            rows_size: cols,
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix_2 {
        let mut res = Matrix_2::zeros(rows, cols);
        res.data = res
            .data
            .iter()
            .map(|_| thread_rng().gen::<f64>() * 2.0 - 1.0)
            .collect();

        res
    }

    pub fn multiply(&self, other: &Matrix_2) -> Matrix_2 {
        if self.rows_size != other.data.len() / other.rows_size {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }

        let mut res: Matrix_2 = Matrix_2::zeros(self.rows_size, other.data.len() / other.rows_size);

        let m = res.data.iter().enumerate().map(|(i, _)| {
            let tmp = (i / self.rows_size) * self.rows_size;

            (tmp..tmp + self.rows_size)
                .enumerate()
                .fold(0.0 as f64, |mut acc, (i2, i3)| {
                    acc += self.data[i3] * other.data[i % self.rows_size + (i2 * self.rows_size)];
                    acc
                })
        });

        res.data = m.collect();
        res
    }

    pub fn add(&self, other: &Matrix_2) -> Matrix_2 {
        if self.rows_size != other.data.len() / other.rows_size {
            panic!("Attempted to add by matrix of incorrect dimensions");
        }

        let mut res: Matrix_2 = Matrix_2::zeros(self.rows_size, other.data.len() / other.rows_size);

        res.data = res
            .data
            .iter()
            .enumerate()
            .map(|(i, _)| self.data[i] + other.data[i])
            .collect();

        res
    }

    pub fn dot_multiply(&self, other: &Matrix_2) -> Matrix_2 {
        if self.rows_size != other.data.len() / other.rows_size {
            panic!("Attempted to add by matrix of incorrect dimensions");
        }

        let mut res: Matrix_2 = Matrix_2::zeros(self.rows_size, other.data.len() / other.rows_size);

        res.data = res
            .data
            .iter()
            .enumerate()
            .map(|(i, _)| self.data[i] * other.data[i])
            .collect();

        res
    }
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();

        let mut res = Matrix::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        res
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }

                res.data[i][j] = sum;
            }
        }

        res
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add matrix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        res
    }

    pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        res
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtract matrix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        res
    }

    pub fn map<F: Fn(f64) -> f64 + Send + Sync>(&self, function: F) -> Matrix {
        Matrix::from(
            (self.data)
                .clone()
                .into_par_iter()
                .map(|row| row.into_par_iter().map(|value| function(value)).collect())
                .collect(),
        )
    }

    pub fn transpose(&self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        res
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Matrix {{\n{}\n}}",
            (&self.data)
                .into_iter()
                .map(|row| "  ".to_string()
                    + &row
                        .into_iter()
                        .map(|value| value.to_string())
                        .collect::<Vec<String>>()
                        .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn matrix_2_multiply() {
        let mut m1 = Matrix_2::zeros(3, 3);
        m1.data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec();

        let mut m2 = Matrix_2::zeros(3, 3);
        m2.data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec();

        let mut res = m1.multiply(&m2);

        let x = std::time::Instant::now();

        for i in 0..1000000 {
            m1.multiply(&m2);
        }

        println!("{:?}", x.elapsed());

        assert_eq!(
            [30.0, 36.0, 42.0, 66.0, 81.0, 96.0, 102.0, 126.0, 150.0].to_vec(),
            res.data
        );
    }

    #[test]
    pub fn matrix_1_vs_matrix_2_multiply() {
        let mut m1 = Matrix::zeros(3, 3);
        m1.data = [
            [1.0, 2.0, 3.0].to_vec(),
            [4.0, 5.0, 6.0].to_vec(),
            [7.0, 8.0, 9.0].to_vec(),
        ]
        .to_vec();

        let mut m2 = Matrix::zeros(3, 3);
        m1.data = [
            [1.0, 2.0, 3.0].to_vec(),
            [4.0, 5.0, 6.0].to_vec(),
            [7.0, 8.0, 9.0].to_vec(),
        ]
        .to_vec();

        let mut res = m1.multiply(&m2);

        let x = std::time::Instant::now();

        for i in 0..1000000 {
            m1.multiply(&m2);
        }

        println!("Matrix_1_time:{:?}", x.elapsed());

        assert_eq!(
            [
                [30.0, 36.0, 42.0].to_vec(),
                [66.0, 81.0, 96.0].to_vec(),
                [102.0, 126.0, 150.0].to_vec()
            ]
            .to_vec(),
            res.data
        );
    }

    #[test]
    pub fn matrix_2_add() {
        let mut m1 = Matrix_2::zeros(3, 3);
        m1.data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec();

        let mut m2 = Matrix_2::zeros(3, 3);
        m2.data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec();

        let mut res = m1.add(&m2);

        let x = std::time::Instant::now();

        for i in 0..1000000 {
            m1.multiply(&m2);
        }

        println!("{:?}", x.elapsed());

        assert_eq!(
            res.data,
            [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0].to_vec()
        )
    }

    #[test]
    pub fn matrix_2_dot_multiply() {
        assert_eq!(1, 1)
    }
}
