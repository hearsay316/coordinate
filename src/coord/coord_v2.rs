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
        // 计算x坐标的差值
        let dx = self.x - other.x;
        // 计算y坐标的差值
        let dy = self.y - other.y;
        // 使用勾股定理计算距离并返回
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
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();
        Self {
            // 生成n个随机点并收集到一个向量中
            coord: (0..n)
                .map(|_| Point {
                    // 生成0到100之间的随机x坐标
                    x: rng.gen_range(0.0..100.0),
                    // 生成0到100之间的随机y坐标
                    y: rng.gen_range(0.0..100.0),
                })
                .collect(),
        }
    }

    /// 找到离目标点最近的m个点
    pub fn find_closest(&self, target: &Point, m: usize) -> Vec<Point> {
        // 创建一个向量来存储每个点及其到目标点的距离
        let mut vec = Vec::new();
        // 遍历坐标系中的所有点
        for point in &self.coord {
            // 计算当前点到目标点的距离
            let dist = point.distance(&target);
            // 将距离和点的克隆添加到向量中
            vec.push((dist, point.clone()));
        }
        // 按距离从小到大排序
        vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        // 取出距离最近的m个点，只返回点而不返回距离
        vec.iter()
            .map(|(_, point)| point.clone())
            .take(m)
            .collect::<Vec<Point>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_coordinate_find_closest() {
        let coord = Coordinate::new(5);
        let target = Point::new(0.0, 0.0);
        let closest_points = coord.find_closest(&target, 3);
        assert!(closest_points.len() <= 3);
        for point in closest_points {
            assert!(point.distance(&target) <= 100.0);
        }
    }
}

