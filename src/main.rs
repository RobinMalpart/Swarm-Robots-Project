mod map;

fn main() {
    // Create a 20x10 map
    let map = map::Map::new(70, 70);

    // Display the ASCII map in the terminal
    map.display();
}
