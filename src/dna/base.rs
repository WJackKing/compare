///碱基

use rand;
use rand::Rng;

///用于储存碱基的结构体
pub struct Base {
    base: usize,
}

impl Base {
    ///生成默认碱基A
    pub fn default() -> Base {
        Base { base: 0usize }
    }

    ///随机生成一个碱基
    pub fn rand() -> Base {
        Base { base: get_rand() }
    }
}

impl Base {
    //从一个数字生成碱基，数字范围为0到3，分别对应ACGT
    pub fn from_num<'a>(base: usize) -> Result<Base, &'a str> {
        match base {
            0...4 => Ok(Base { base }),
            _ => Err("u8 error"),
        }
    }
}

impl Base {
    //获取碱基的编号
    pub fn get_num(&self) -> usize {
        self.base
    }
}

impl Clone for Base {
    fn clone(&self) -> Self {
        Base { base: self.base }
    }
}

///获取任意数方法，范围为0到3
pub fn get_rand() -> usize {
    rand::thread_rng().gen_range(0, 4)
}
