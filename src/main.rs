use coordinate::coord::use_rstar::Point;

// coordinate
fn main() {
    let points = Point::new(10);
    let target = [50.0, 50.0]; // 目标坐标，可以按需修改
    let m = 3; // 要获取的最近坐标数量，可以按需修改
    let closest_points = points.find_closest(&target, m);
    println!("随机生成的坐标集合: {:?}", points);
    println!(
        "距离目标坐标 最近的 {} 个坐标: {:?}",
         m, closest_points
    );
}

