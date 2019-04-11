use super::structs::{Coordinate, Cost, Point};
use std::collections::{HashMap, HashSet};

pub struct Grid {
    width: i16,
    height: i16,
    walls: HashSet<Coordinate>,
}

impl Grid {
    pub fn new(lines: &[String]) -> Self {
        let walls = find_walls(lines);
        let height = lines.len() as i16;
        let width = lines[0].len() as i16;

        Grid {
            width,
            height,
            walls,
        }
    }

    pub fn neighbors(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        let neighbor_coordinates = vec![
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y + 1,
            },
            Coordinate {
                x: coordinate.x - 1,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            },
        ];

        let mut valid_coordinates: Vec<Coordinate> = Vec::new();

        for neighbor_coordinate in neighbor_coordinates {
            if self.is_passable(neighbor_coordinate) {
                valid_coordinates.push(neighbor_coordinate);
            }
        }

        valid_coordinates.to_vec()
    }

    fn is_passable(&self, coordinate: Coordinate) -> bool {
        let is_in_boundaries = coordinate.x < self.width && coordinate.y < self.height;

        is_in_boundaries && !self.walls.contains(&coordinate)
    }
}

fn find_walls(map_lines: &[String]) -> HashSet<Coordinate> {
    let mut walls = HashSet::new();

    for (y, line) in map_lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == 'x' {
                walls.insert(Coordinate {
                    x: x as i16,
                    y: y as i16,
                });
            }
        }
    }

    walls
}

#[derive(Debug)]
enum Tile {
    Passable(u8),
    Impassable,
}

fn char_to_tile(character: char) -> Tile {
    match character {
        '-' => Tile::Passable(1),
        'o' => Tile::Passable(5),
        'h' => Tile::Passable(10),
        'e' => Tile::Passable(1),
        'x' => Tile::Impassable,
        _ => Tile::Impassable,
    }
}

pub struct GridWithCosts {
    width: i16,
    height: i16,
    costs: HashMap<Coordinate, Cost>,
    walls: HashSet<Coordinate>,
}

impl GridWithCosts {
    pub fn new(lines: &[String]) -> Self {
        let mut costs = HashMap::new();
        let mut walls = HashSet::new();
        let height = lines.len() as i16;
        let width = lines[0].len() as i16;

        for (y, chars) in lines.iter().enumerate() {
            for (x, char_point) in chars.chars().enumerate() {
                let coordinate = Coordinate {
                    x: x as i16,
                    y: y as i16,
                };
                let tile = char_to_tile(char_point);

                match tile {
                    Tile::Passable(cost) => {
                        costs.insert(coordinate, cost);
                    }
                    Tile::Impassable => {
                        walls.insert(coordinate);
                    }
                };
            }
        }

        Self {
            width,
            height,
            costs,
            walls,
        }
    }

    pub fn neighbors(&self, coordinate: Coordinate) -> Vec<Point> {
        let neighbor_coordinates = vec![
            Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y + 1,
            },
            Coordinate {
                x: coordinate.x - 1,
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            },
        ];

        let mut points: Vec<Point> = Vec::new();

        for neighbor_coordinate in neighbor_coordinates {
            if self.is_passable(neighbor_coordinate) {
                let coordinate = Coordinate {
                    x: neighbor_coordinate.x,
                    y: neighbor_coordinate.y,
                };

                if let Some(cost) = self.costs.get(&coordinate) {
                    points.push(Point {
                        coordinate,
                        cost: *cost,
                    })
                }
            }
        }

        points
    }

    fn is_passable(&self, coordinate: Coordinate) -> bool {
        let is_in_boundaries = coordinate.x < self.width && coordinate.y < self.height;

        is_in_boundaries && !self.walls.contains(&coordinate)
    }
}

pub fn find_end(map_lines: &[String]) -> Option<Coordinate> {
    let mut end: Option<Coordinate> = None;

    for (y, line) in map_lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == 'e' {
                end = Some(Coordinate {
                    x: x as i16,
                    y: y as i16,
                });
            }
        }
    }

    end
}

pub fn absolute_distance(from: Coordinate, to: Coordinate) -> i16 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
}
