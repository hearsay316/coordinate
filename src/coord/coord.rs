use rand::Rng;

/// 表示二维空间中的一个点。
#[derive(Debug, Clone)]
pub struct Point {
    x: PointVal,
    y: PointVal,
}

impl Point {
    /// 创建一个新的 `Point`。
    ///
    /// # 参数
    ///
    /// - `x`: x 坐标值。
    /// - `y`: y 坐标值。
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: PointVal::from_f64(x),
            y: PointVal::from_f64(y),
        }
    }

    /// 计算当前点与另一个点之间的距离。
    ///
    /// # 参数
    ///
    /// - `other`: 另一个点。
    ///
    /// # 返回值
    ///
    /// 返回到另一个点的距离，以 `PointVal` 形式表示。
    fn distance(&self, other: &Point) -> PointVal {
        let dx = self.x.to_f64() - other.x.to_f64();
        let dy = self.y.to_f64() - other.y.to_f64();
        PointVal::from_f64((dx * dx + dy * dy).sqrt())
    }
}

/// 表示一个坐标值，分为整数部分和小数部分。
#[derive(Debug, Clone)]
struct PointVal {
    integral: String,
    fractional: String,
}

impl PointVal {
    /// 从整数部分和小数部分创建一个新的 `PointVal`。
    ///
    /// # 参数
    ///
    /// - `i`: 整数部分。
    /// - `f`: 小数部分。
    fn new(i: u64, f: u64) -> Self {
        Self {
            integral: i.to_string(),
            fractional: f.to_string(),
        }
    }

    /// 从表示整数部分和小数部分的字符串创建一个新的 `PointVal`。
    ///
    /// # 参数
    ///
    /// - `i`: 表示整数部分的字符串。
    /// - `f`: 表示小数部分的字符串。
    fn from_string(i: String, f: String) -> Self {
        Self {
            integral: i,
            fractional: f,
        }
    }

    /// 创建一个随机的 `PointVal`。
    fn rng_new() -> PointVal {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..100);
        let f = rng.gen_range(0..100);
        PointVal::new(i, f)
    }

    /// 从 `f64` 值创建一个 `PointVal`。
    ///
    /// # 参数
    ///
    /// - `f`: 要转换的 `f64` 值。
    fn from_f64(f: f64) -> PointVal {
        let binding = f.to_string();
        let mut vec = binding.split('.').collect::<Vec<_>>();
        // 补零
        if vec.len() == 1 {
            vec.push("0");
        }
        PointVal::from_string(vec[0].into(), vec[1].into())
    }

    /// 将 `PointVal` 转换为 `f64`。
    fn to_f64(&self) -> f64 {
        format!("{}.{}", self.integral, self.fractional)
            .parse::<f64>()
            .unwrap()
    }

    /// 将 `PointVal` 转换为 `u64`，按指定长度进行缩放。
    ///
    /// # 参数
    ///
    /// - `len`: 缩放长度。
    fn to_u64(&self, len: i64) -> u64 {
        let i = self.integral.parse::<u64>().unwrap();
        let num = (0..len).fold(1, |acc, _| acc * 10);
        i * num + self.fractional.parse::<u64>().unwrap()
    }
}

/// 表示二维空间中的一个点的集合。
#[derive(Debug)]
pub struct Coordinate {
    coord: Vec<Point>,
}

impl Coordinate {
    /// 创建一个新的 `Coordinate`，包含指定数量的随机点。
    ///
    /// # 参数
    ///
    /// - `n`: 要生成的点的数量。
    pub fn new(n: usize) -> Self {
        Self {
            coord: (0..n)
                .map(|_| Point {
                    x: PointVal::rng_new(),
                    y: PointVal::rng_new(),
                })
                .collect(),
        }
    }

    /// 查找距离目标点最近的点。
    ///
    /// # 参数
    ///
    /// - `target`: 目标点。
    /// - `m`: 要查找的最近点的数量。
    ///
    /// # 返回值
    ///
    /// 返回包含最近点的向量。
    pub fn find_closest(&self, target: &Point, m: usize) -> Vec<Point> {
        let mut vec = Vec::new();
        let mut length = 1;
        for point in &self.coord {
            let dist = point.distance(&target);
            if dist.fractional.len() > length {
                length = dist.fractional.len();
            }
            vec.push((dist, point.clone()));
        }
        // 排序从小到大
        vec.sort_by(|a, b| a.0.to_u64(length as i64).cmp(&b.0.to_u64(length as i64)));
        // 取出前 m 个数据
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
    fn test_point_new() {
        let point = Point::new(1.0, 2.0);
        assert_eq!(point.x.to_f64(), 1.0);
        assert_eq!(point.y.to_f64(), 2.0);
    }

    #[test]
    fn test_point_distance() {
        let point1 = Point::new(1.0, 2.0);
        let point2 = Point::new(4.0, 6.0);
        assert_eq!(point1.distance(&point2).to_f64(), 5.0);
    }

    #[test]
    fn test_point_val_new() {
        let point_val = PointVal::new(1, 2);
        assert_eq!(point_val.integral, "1");
        assert_eq!(point_val.fractional, "2");
    }

    #[test]
    fn test_point_val_from_string() {
        let point_val = PointVal::from_string("1".to_string(), "2".to_string());
        assert_eq!(point_val.integral, "1");
        assert_eq!(point_val.fractional, "2");
    }

    #[test]
    fn test_point_val_from_f64() {
        let point_val = PointVal::from_f64(1.2);
        assert_eq!(point_val.integral, "1");
        assert_eq!(point_val.fractional, "2");
    }

    #[test]
    fn test_point_val_to_f64() {
        let point_val = PointVal::new(1, 2);
        assert_eq!(point_val.to_f64(), 1.2);
    }

    #[test]
    fn test_point_val_to_u64() {
        let point_val = PointVal::new(1, 2);
        assert_eq!(point_val.to_u64(2), 102);
    }

    #[test]
    fn test_coordinate_new() {
        let coordinate = Coordinate::new(10);
        assert_eq!(coordinate.coord.len(), 10);
    }

    #[test]
    fn test_coordinate_find_closest() {
        let coordinate = Coordinate::new(10);
        let target = Point::new(1.0, 2.0);
        let closest_points = coordinate.find_closest(&target, 3);
        assert_eq!(closest_points.len(), 3);
    }
}
