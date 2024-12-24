// 使用rand crate来生成随机数
use rand::Rng;
// 使用rstar crate来使用R树数据结构
use rstar::RTree;

/// Point结构体，包含一个RTree实例，用于存储二维坐标点
#[derive(Debug)]
pub struct Point {
    pub vec: RTree<[f64; 2]>,
}

impl Point {
    /// 创建一个新的Point实例，包含指定数量的随机点
    ///
    /// # 参数
    ///
    /// * `i` - 要生成的随机点的数量
    ///
    /// # 返回值
    ///
    /// 返回一个Point实例，其中包含一个RTree，树中插入了`i`个随机生成的二维点
    pub fn new(i: usize) -> Self {
        let mut vec = RTree::new();
        let mut rng = rand::thread_rng();
        for _ in 0..i {
            let x = rng.gen_range(0.0..100.0);
            let y = rng.gen_range(0.0..100.0);
            vec.insert([x, y]);
        }
        Self { vec }
    }

    /// 查找并返回离目标点最近的指定数量的点
    ///
    /// # 参数
    ///
    /// * `target` - 目标点的坐标
    /// * `m` - 要返回的最近点的数量
    ///
    /// # 返回值
    ///
    /// 返回一个向量，包含离目标点最近的`m`个点的引用
    pub fn find_closest(&self, target: &[f64; 2], m: usize) -> Vec<&[f64; 2]> {
        self.vec
            .nearest_neighbor_iter(target)
            .take(m)
            .collect::<Vec<_>>()
    }
}
