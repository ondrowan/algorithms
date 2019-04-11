use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::VecDeque;

use super::grid::{find_end, Grid};
use super::structs::{Coordinate, VisitedCoordinates};

pub fn breadth_first(map: &[String], start_coordinate: Coordinate) -> Option<VisitedCoordinates> {
    let grid = Grid::new(&map);
    let end_coordinate = find_end(&map).unwrap();

    let mut frontier: VecDeque<Coordinate> = VecDeque::new();
    frontier.push_back(start_coordinate);

    let mut visited: VisitedCoordinates = VisitedCoordinates::new();
    visited.insert(start_coordinate, None);

    while !frontier.is_empty() {
        let current_coordinate = frontier.pop_front().unwrap();
        let neighbors = grid.neighbors(current_coordinate);

        if current_coordinate == end_coordinate {
            return Some(visited);
        }

        for next_coordinate in neighbors {
            match visited.entry(next_coordinate) {
                Vacant(visited_coordinate) => {
                    frontier.push_back(next_coordinate);
                    visited_coordinate.insert(Some(current_coordinate));
                }
                Occupied(_visited_coordinate) => {}
            };
        }
    }

    None
}
