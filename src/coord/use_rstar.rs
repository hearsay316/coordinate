// 引入随机数生成器模块
use rand::Rng;
// 引入R*树模块
use rstar::RTree;

// 定义一个公共的结构体Point，包含一个RTree类型的vec字段
#[derive(Debug)]
pub struct Point {
    pub vec: RTree<[f64; 2]>,
}

// 为Point结构体实现方法
impl Point {

    // 定义一个公共的构造函数new，接收一个usize类型的参数i
    pub fn new(i: usize) -> Self {
        // 创建一个新的RTree实例
        let mut vec = RTree::new();
        // 获取一个线程局部的随机数生成器
        let mut rng = rand::thread_rng();
        // 循环i次
        for _ in 0..i {
            // 生成一个0到100之间的随机数作为x坐标
            let x = rng.gen_range(0.0..100.0);
            // 生成一个0到100之间的随机数作为y坐标
            let y = rng.gen_range(0.0..100.0);
            // 将坐标[x, y]插入到RTree中
            vec.insert([x, y]);
        }
        // 返回一个包含RTree的Point实例
        Self { vec }
    }

    // 定义一个公共的方法find_closest，接收一个目标坐标target和一个usize类型的参数m
    pub fn find_closest(&self, target: &[f64; 2], m: usize) -> Vec<&[f64; 2]> {
        // 在RTree中查找最近的m个邻居节点，并将结果收集到一个Vec中返回
        self.vec
            .nearest_neighbor_iter(target)
            .take(m)
            .collect::<Vec<_>>()
    }
}
