pub mod base;

use self::base::Base;
use self::base::get_rand;

pub static BASE: [char; 4] = ['A', 'C', 'G', 'T'];

///储存base序列的结构体
pub struct Dna {
    pub genes: Vec<Base>,
}

impl Dna {
    ///根据所给长度生成一段多聚A序列
    pub fn new(len: usize) -> Dna {
        Dna {
            genes: vec![Base::default(); len],
        }
    }

    ///根据所给长度生成一段随机序列
    pub fn new_rand(len: usize) -> Dna {
        let mut dna = Dna {
            genes: vec![Base::rand(); 0],
        };
        for _i in 0..len {
            dna.genes.push(Base::rand());
        }
        dna
    }
}

impl Dna {
    ///从所给数组生成dna序列，数组元素取值范围为0到3
    pub fn from_array(bases: Vec<usize>) -> Dna {
        let mut dna = Dna::new(0);
        for base in bases {
            match base {
                0...4 => dna.push_num(base).unwrap(),
                _ => (),
            }
        }
        dna
    }
}

impl Dna {
    ///追加一个碱基
    pub fn push_num<'a>(&mut self, base: usize) -> Result<(), &'a str> {
        match Base::from_num(base) {
            Ok(gene) => Ok(self.genes.push(gene)),
            Err(err) => Err(err),
        }
    }

    ///追加一个dna序列
    pub fn append(&mut self, other: &Dna) {
        for i in 0..other.len() {
            self.push_num(other.index_num(i)).unwrap();
        }
    }
}

impl Dna {
    ///改变index处的碱基
    pub fn set_num<'a>(&mut self, index: usize, base: usize) -> Result<(), &'a str> {
        match Base::from_num(base) {
            Ok(gene) => Ok(self.genes[index] = gene),
            Err(err) => Err(err),
        }
    }
}

impl Dna {
    ///获取index处的碱基
    pub fn index(&self, index: usize) -> Base {
        self.genes[index].clone()
    }

    ///获取index处的碱基所对应的编号
    pub fn index_num(&self, index: usize) -> usize {
        self.genes[index].get_num()
    }

    ///获取dna链长度
    pub fn len(&self) -> usize {
        self.genes.len()
    }
}

impl Dna {
    ///查找一个碱基
    pub fn find_base(&self, base: &Base, offset: usize) -> Option<usize> {
        for i in offset..self.len() {
            if self.genes[i].get_num() == base.get_num() {
                return Some(i);
            }
        }
        return None;
    }

    ///查找所有碱基
    pub fn find_base_all(&self, base: &Base, offset: usize) -> Option<Vec<usize>> {
        let mut v = vec![0usize; 0];
        for i in offset..self.len() {
            if self.genes[i].get_num() == base.get_num() {
                v.push(i);
            }
        }
        if v.len() != 0 {
            Some(v)
        } else {
            None
        }
    }
}

impl Dna {
    ///交叉
    pub fn crossover(mut self, mut other: Dna, start: usize, end: usize) -> (Dna, Dna) {
        let mut base_tmp;
        for i in start..end {
            base_tmp = self.index_num(i);
            self.set_num(i, other.index_num(i)).unwrap();
            other.set_num(i, base_tmp).unwrap();
        }
        (self, other)
    }

    ///变异
    pub fn variation(&mut self, index: usize) {
        let base = self.index(index).get_num();
        let mut new: usize;
        loop {
            new = get_rand();
            if new != base {
                break;
            }
        }
        self.set_num(index, new).unwrap();
    }
}

impl Dna {
    ///清空dna序列
    pub fn clean(&mut self) {
        self.genes = vec![Base::default(); 0];
    }
}

impl Clone for Dna {
    fn clone(&self) -> Self {
        Dna {
            genes: self.genes.clone(),
        }
    }
}

///输出dna
pub fn print_dna(dna: &Dna) {
    for i in 0..dna.len() {
        print!("{}", BASE[dna.index_num(i)]);
    }
}
