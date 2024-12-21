use rand::Rng;

#[derive(Debug, Clone)]
pub struct Point {
    x: PointVal,
    y: PointVal,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: PointVal::from_f64(x),
            y: PointVal::from_f64(y),
        }
    }
    fn distance(&self, other: &Point) -> PointVal {
        let dx = self.x.to_f64() - other.x.to_f64();
        let dy = self.y.to_f64() - other.y.to_f64();
        PointVal::from_f64((dx * dx + dy * dy).sqrt())
    }
}
#[derive(Debug, Clone)]
struct PointVal {
    integral: String,
    fractional: String,
}

impl PointVal {
    fn new(i: u64, f: u64) -> Self {
        Self {
            integral: i.to_string(),
            fractional: f.to_string(),
        }
    }
    fn from_string(i: String, f: String) -> Self {
        Self {
            integral: i,
            fractional: f,
        }
    }
    fn rng_new() -> PointVal {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..100);
        let f = rng.gen_range(0..100);
        PointVal::new(i, f)
    }
    fn from_f64(f: f64) -> PointVal {
        let binding = f.to_string();
        let mut vec = binding.split('.').collect::<Vec<_>>();
        //补零
        if vec.len() == 1 {
            vec.push("0");
        }
        PointVal::from_string(vec[0].into(), vec[1].into())
    }
    fn to_f64(&self) -> f64 {
        format!("{}.{}", self.integral, self.fractional)
            .parse::<f64>()
            .unwrap()
    }

    fn to_u64(&self, len: i64) -> u64 {
        let i = self.integral.parse::<u64>().unwrap();
        let num = (0..len).fold(1, |acc, _| acc * 10);
        i * num + self.fractional.parse::<u64>().unwrap()
    }
}

#[derive(Debug)]
pub struct Coordinate {
    coord: Vec<Point>,
}

impl Coordinate {
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
        //排序小到大
        vec.sort_by(|a, b| a.0.to_u64(length as i64).cmp(&b.0.to_u64(length as i64)));
        // 取出来m个数据
        vec.iter()
            .map(|(_, point)| point.clone())
            .take(m)
            .collect::<Vec<Point>>()
    }
}
