use model::data::Node;

type Distance = u32;

pub fn euclidean2d(from: &Node, to: &Node) -> Distance {
    let xd = from.x - to.x;
    let yd = from.y - to.y;
    let floatingPointDistance = xd.hypot(yd);
    floatingPointDistance.round() as Distance
}

pub fn euclidean3d(from: &Node, to: &Node) -> Distance {
    let xd = from.x - to.x;
    let yd = from.y - to.y;
    let zd = from.z - to.z;
    let distanceSquared = xd * xd + yd * yd + zd * zd;
    let floatingPointDistance = distanceSquared.sqrt();
    floatingPointDistance.round() as Distance
}

pub fn manhattan_2d(from: &Node, to: &Node) -> Distance {
    let xd = (from.x - to.x).abs();
    let yd = (from.y - to.y).abs();
    (xd + yd).round() as Distance
}

pub fn manhattan_3d(from: &Node, to: &Node) -> Distance {
    let xd = (from.x - to.x).abs();
    let yd = (from.y - to.y).abs();
    let zd = (from.z - to.z).abs();
    (xd + yd + zd).round() as Distance
}

pub fn maximum_2d(from: &Node, to: &Node) -> Distance {
    let xd = (from.x - to.x).abs();
    let yd = (from.y - to.y).abs();
    xd.round().max(yd.round()) as Distance
}

pub fn maximum_3d(from: &Node, to: &Node) -> Distance {
    let xd = (from.x - to.x).abs();
    let yd = (from.y - to.y).abs();
    let zd = (from.z - to.z).abs();
    xd.round().max(yd.round()).max(zd.round()) as Distance
}

const PI: f64 = 3.141592;

fn degrees_to_radians(degrees_and_minutes: f64) -> f64 {
    let degrees = degrees_and_minutes.round();
    let minutes = degrees_and_minutes - degrees;
    PI * (degrees + 5f64 * minutes / 3f64) / 180f64
}

const RRR: f64 = 6378.388;
pub fn geographical(from: &Node, to: &Node) -> Distance {
    let longitude1 = degrees_to_radians(from.y);
    let longitude2 = degrees_to_radians(to.y);
    let latitude1 = degrees_to_radians(from.x);
    let latitude2 = degrees_to_radians(to.x);
    let q1 = (longitude1 - longitude2).cos();
    let q2 = (latitude1 - latitude2).cos();
    let q3 = (latitude1 + latitude2).cos();
    (RRR * (0.5f64 * ((1f64 + q1) * q2 - (1f64 - q1) * q3)).acos() + 1f64) as Distance
}

pub fn pseude_euclidean(from: &Node, to: &Node) -> Distance {
    let xd = from.x - to.x;
    let yd = from.y - to.y;
    let rij = ((xd * xd + yd * yd) / 10f64).sqrt();
    let tij = rij.round();
    if tij < rij {
        tij as Distance + 1
    } else {
        tij as Distance
    }
}
