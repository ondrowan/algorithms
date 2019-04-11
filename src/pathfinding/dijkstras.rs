use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BinaryHeap;

use super::grid::{find_end, GridWithCosts};
use super::structs::{Coordinate, CostsSoFar, VisitedCoordinates, VisitedPoint};

pub fn dijkstras(map: &[String], start_coordinate: Coordinate) -> Option<VisitedCoordinates> {
    let grid = GridWithCosts::new(&map);
    let end_coordinate = find_end(&map).unwrap();
    let start_point = VisitedPoint {
        coordinate: start_coordinate,
        cost_so_far: 0,
    };

    let mut frontier = BinaryHeap::new();
    frontier.push(start_point);

    let mut visited = VisitedCoordinates::new();
    visited.insert(start_coordinate, None);
    let mut costs_so_far = CostsSoFar::new();
    costs_so_far.insert(start_coordinate, 0);

    while !frontier.is_empty() {
        let current_point = frontier.pop().unwrap();
        let neighbors = grid.neighbors(current_point.coordinate);

        if current_point.coordinate == end_coordinate {
            return Some(visited);
        }

        for next_point in neighbors {
            let new_cost = costs_so_far[&current_point.coordinate] + u16::from(next_point.cost);

            match visited.entry(next_point.coordinate) {
                Vacant(_visited_coordinate) => {
                    frontier.push(VisitedPoint {
                        coordinate: next_point.coordinate,
                        cost_so_far: new_cost,
                    });
                    visited.insert(next_point.coordinate, Some(current_point.coordinate));
                    costs_so_far.insert(next_point.coordinate, new_cost);
                }
                Occupied(mut visited_coordinate) => {
                    let visited_cost_so_far = costs_so_far[visited_coordinate.key()];

                    if visited_cost_so_far > new_cost {
                        frontier.push(VisitedPoint {
                            coordinate: next_point.coordinate,
                            cost_so_far: new_cost,
                        });
                        visited_coordinate.insert(Some(current_point.coordinate));
                        costs_so_far.insert(next_point.coordinate, new_cost);
                    }
                }
            };
        }
    }

    None
}
