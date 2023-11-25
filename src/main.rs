use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::lib::{network::Network, matrix::Matrix_2};

pub mod lib;

fn main() {
    
    fn matrix_2_multiply() {
        let mut m1 = Matrix_2::zeros(3, 3);
        m1.data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec();

        let mut m2 = Matrix_2::zeros(3, 3);
        m2.data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec();

        let mut res = m1.multiply(&m2);

        let x = std::time::Instant::now();

        (0..1000000).into_par_iter().for_each(|_| {
            m1.multiply(&m2);
        });

        println!("{:?}", x.elapsed());

        assert_eq!(
            [30.0, 36.0, 42.0, 66.0, 81.0, 96.0, 102.0, 126.0, 150.0].to_vec(),
            res.data
        );
    }

    println!("multi thread");
    matrix_2_multiply();

    //let inputs = vec![
    //    vec![0.0, 0.0],
    //    vec![0.0, 1.0],
    //    vec![1.0, 0.0],
    //    vec![1.0, 1.0],
    //];

    //let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

    //let mut network = Network::new(vec![2, 1000, 1], 0.5);

    //network.train(inputs, targets, std::u16::MAX);

    //println!("0 and 0: {:?}", network.feed_forward(vec![0.0, 0.0]));
    //println!("0 and 1: {:?}", network.feed_forward(vec![0.0, 1.0]));
    //println!("1 and 0: {:?}", network.feed_forward(vec![1.0, 0.0]));
    //println!("1 and 1: {:?}", network.feed_forward(vec![1.0, 1.0]));
}
