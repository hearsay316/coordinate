use rand::Rng;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug)]
pub struct Coordinate {
    coord: Vec<Point>,
}

impl Coordinate {
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

    pub fn find_closest(&self, target: &Point, m: usize) -> Vec<Point> {
        let mut vec = Vec::new();
        for point in &self.coord {
            let dist = point.distance(&target);
            vec.push((dist, point.clone()));
        }
        //排序小到大
        vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        println!("{:?}", vec);
        // 取出来m个数据
        vec.iter()
            .map(|(_, point)| point.clone())
            .take(m)
            .collect::<Vec<Point>>()
    }
}
