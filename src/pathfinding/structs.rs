use std::cmp::Ordering;
use std::collections::HashMap;

/// Coordinate on a map. Although neither x or y can be less than 0, i16 was
/// chosen as results of some arithmetic operations may produce negative numbers.
///
/// +--x
/// |
/// y
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type Cost = u8;

/// Representation of map coordinate and cost of passing through it.
#[derive(Debug)]
pub struct Point {
    pub coordinate: Coordinate,
    pub cost: Cost,
}

/// This might seem redundant, but it's needed as BinaryHeap supports max-heap
/// only, so we need to flip the comparison between costs of already visited
/// points.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct VisitedPoint {
    pub coordinate: Coordinate,
    pub cost_so_far: u16,
}

impl Ord for VisitedPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost_so_far
            .cmp(&self.cost_so_far)
            .then_with(|| self.coordinate.cmp(&other.coordinate))
    }
}

impl PartialOrd for VisitedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// <To, From>
pub type VisitedCoordinates = HashMap<Coordinate, Option<Coordinate>>;
pub type CostsSoFar = HashMap<Coordinate, u16>;
