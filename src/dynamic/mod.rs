use dna::Dna;
use dna::BASE;

pub struct Score{
    score: i32,
    coordinate: (usize, usize),
}

impl Score{
    pub fn new(score: i32, coordinate: (usize, usize)) -> Score{
        Score{
            score,
            coordinate,
        }
    }

    pub fn default() -> Score{
        Score{
            score: 0,
            coordinate: (0, 0),
        }
    }

    pub fn s(&self) -> i32{
        self.score
    }

    pub fn c(&self) -> (usize, usize){
        self.coordinate
    }
}

impl Clone for Score{
    fn clone(&self) -> Self{
        Score{
            score: self.score,
            coordinate: self.coordinate,
        }
    }
}

pub fn dynamic(v1: &Dna, v2: &Dna, matrix: &Vec<Vec<i32>>, d: i32) -> Vec<Vec<Score>>{
    let mut sum_matrix = vec![vec![Score::default(); v1.len() + 1]; v2.len() + 1];

    sum_matrix[0][0] = Score::new(0, (0, 0));
    for i in 1..sum_matrix.len() {
        sum_matrix[i][0] = Score::new(-5 * i as i32, (i - 1, 0));
    }
    for i in 1..sum_matrix[0].len(){
        sum_matrix[0][i] = Score::new(-5 * i as i32, (0, i - 1));
    }
    for i in 1..sum_matrix.len() {
        for j in 1..sum_matrix[0].len() {
            sum_matrix[i][j] = max(
                sum_matrix[i][j - 1].s() + d,
                sum_matrix[i - 1][j].s() + d,
                sum_matrix[i - 1][j - 1].s() + matrix[v2.index_num(i - 1)][v1.index_num(j - 1)],
                i,
                j
            );
        }
    }
    sum_matrix
}

pub fn dynamic_print(v1: &Dna, v2: &Dna, matrix: &Vec<Vec<i32>>, d: i32){
    let sum_matrix = dynamic(v1, v2, matrix, d);
    let mut xy = (v2.len(), v1.len());
    let mut xv = vec![0usize; 0];
    let mut yv = vec![0usize; 0];
    loop{
        if xy.0 == 0 && xy.1 == 0{
            break;
        }
        xv.push(xy.0);
        yv.push(xy.1);
        xy = sum_matrix[xy.0][xy.1].c();
    }
    let mut n = 0;
    for i in 0..yv.len(){
        if n == yv[yv.len() - i - 1]{
            print!("-");
        }else{
            n = yv[yv.len() - i - 1];
            if n > 0{
                //print array 1
                print!("{}", BASE[v1.index_num(n - 1)]);
            }
        }
    }
    println!();
    n = 0;
    for i in 0..xv.len(){
        if n == xv[xv.len() - i - 1]{
            print!("-");
        }else{
            n = xv[xv.len() - i - 1];
            if n > 0{
                //print array 2
                print!("{}", BASE[v2.index_num(n - 1)]);
            }
        }
    }
    println!();
}

fn max(a: i32, b: i32, c: i32, i: usize, j: usize) -> Score {
    if c >= a && c >= b{
        Score::new(c, (i - 1, j - 1))
    }else if b >= a && b >= c{
        Score::new(b, (i - 1, j))
    }else{
        Score::new(a, (i, j - 1))
    }
}