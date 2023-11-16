use std::f64::consts::E;

/*#[derive(Clone)]
pub struct Activation {
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    function: &|x| 1.0 / (1.0 + E.powf(-x)),
    derivative: &|x| x * (1.0 - x),
};*/

pub struct Activation {
}

impl Activation {

    pub fn sigmoid_function(x: f64) -> f64 {
        1.0/ (1.0 + E.powf(-x))
    }

    pub fn sigmoid_derivative(x: f64) -> f64 {
        x * (1.0 - x)
    }

}

