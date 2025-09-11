use aoc_geometry::{Point, Vector};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SensorInfo {
    sensor_position: Point<i32, 2>,
    beacon_position: Point<i32, 2>,
    distance: u64,
}

impl SensorInfo {
    pub fn new(sensor_position: Point<i32, 2>, beacon_position: Point<i32, 2>) -> Self {
        let v = Vector::<i64, 2>::from_points(&sensor_position, &beacon_position).unwrap();
        let distance = v.manhattan_distance();
        SensorInfo {
            sensor_position,
            beacon_position,
            distance,
        }
    }

    pub fn sensor_position(&self) -> &Point<i32, 2> {
        &self.sensor_position
    }

    pub fn beacon_position(&self) -> &Point<i32, 2> {
        &self.beacon_position
    }

    pub fn distance(&self) -> u64 {
        self.distance
    }
}
