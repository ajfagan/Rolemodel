use rv::dist::InvWishart;
use rv::nalgebra::DMatrix;
use rv::traits::Rv;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    let sigma = DMatrix::from_diagonal_element(20000, 20000, 1.0);
    let wish = InvWishart::new(sigma, 20000).unwrap();

    let mut rng = thread_rng();
    (0..100).for_each(|i| {
        let now = Instant::now();
        let a = wish.draw(&mut rng);
        //let b = a.pseudo_inverse(0.000000001);
        let b = 1.0 / a.determinant();
        println!("{:?}", now.elapsed());
        if i == 99 {
            println!("{:?}", b);
        }
    })
}
