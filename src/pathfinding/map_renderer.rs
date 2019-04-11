use super::grid::find_end;
use super::structs::{Coordinate, VisitedCoordinates};

pub fn render_text(
    mut map: Vec<String>,
    visited: &VisitedCoordinates,
    start_coordinate: Coordinate,
) {
    let mut current_coordinate = find_end(&map).unwrap();
    let mut path: Vec<Coordinate> = Vec::new();

    while current_coordinate != start_coordinate {
        path.push(current_coordinate);
        current_coordinate = visited[&current_coordinate].unwrap();
    }

    path.push(start_coordinate);

    for coordinate in path {
        let x = coordinate.x as usize;
        let y = coordinate.y as usize;

        map[y].replace_range(x..=x, "&");
    }

    for line in map {
        println!("{}", line);
    }
}
