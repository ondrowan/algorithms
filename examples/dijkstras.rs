use algorithms::pathfinding::{dijkstras, file, map_renderer, structs};

fn main() {
    let map: Vec<String> = file::read_map("./examples/map.txt");

    let start_coordinate = structs::Coordinate { x: 0, y: 0 };
    let path = dijkstras::dijkstras(&map, start_coordinate);

    match path {
        Some(path) => map_renderer::render_text(map, &path, start_coordinate),
        None => println!("No path was found!"),
    }
}
