extern crate rand;

pub mod dna;
pub mod dynamic;

use dna::Dna;

pub fn run(){
    let d = -5;
    //罚分矩阵
    let v = vec![
        vec![2, -7, -5, -7],
        vec![-7, 2, -7, -5],
        vec![-5, -7, 2, -7],
        vec![-7, -5, -7, 2],
    ];

    let d1 = Dna::new_rand(99);
    let d2 = Dna::new_rand(99);
    dynamic::dynamic_print(&d1, &d2, &v, d);

    let matrix = dynamic::dynamic(&d1, &d2, &v, d);
    println!("{}", matrix[matrix.len()-1][matrix[0].len()-1].s());
}