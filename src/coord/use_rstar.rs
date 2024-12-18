use rand::Rng;
use rstar::RTree;

#[derive(Debug)]
pub struct Point {
    pub vec: RTree<[f64; 2]>,
}
impl Point {
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
    pub fn find_closest(&self, target: &[f64; 2], m: usize) -> Vec<&[f64; 2]> {
        self.vec
            .nearest_neighbor_iter(target)
            .take(m)
            .collect::<Vec<_>>()
    }
}
