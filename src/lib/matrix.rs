use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{fmt::{Debug, Formatter, Result}, sync::mpsc};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
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

        struct Matrix_Sender {
            row: usize,
            col: usize,
            data: f64
        }

        let mut res = Matrix::zeros(self.rows, other.cols);
        let (tx,  rx) = mpsc::channel::<Matrix_Sender>();
         

        (0..self.rows).into_par_iter().for_each(|i| {
            (0..other.cols).into_par_iter().for_each(|j| {
                (0..self.cols).into_par_iter().for_each(|k| {
                    tx.send(Matrix_Sender { row: i, col: j, data: self.data[i][k] * other.data[k][j] }).unwrap();
                })
                 
            })
        }); 

        drop(tx);

        while let Ok(x) = rx.recv() {
            res.data[x.row][x.col] += x.data;
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
