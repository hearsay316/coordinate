use rand::Rng;

/// 代表二维空间中的一个点
#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// 创建一个新的点
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// 计算当前点与另一个点之间的欧氏距离
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// 代表一个坐标系，其中包含多个点
#[derive(Debug)]
pub struct Coordinate {
    coord: Vec<Point>,
}

impl Coordinate {
    /// 创建一个新的坐标系，包含n个随机生成的点
    pub fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            coord: (0..n)
                .map(|_| Point {
                    x: rng.gen_range(0.0..100.0),
                    y: rng.gen_range(0.0..100.0),
                })
                .collect(),
        }
    }

    /// 找到离目标点最近的m个点
    pub fn find_closest(&self, target: &Point, m: usize) -> Vec<Point> {
        let mut vec = Vec::new();
        for point in &self.coord {
            let dist = point.distance(&target);
            vec.push((dist, point.clone()));
        }
        // 按距离从小到大排序
        vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        // 取出距离最近的m个点
        vec.iter()
            .map(|(_, point)| point.clone())
            .take(m)
            .collect::<Vec<Point>>()
    }
}
