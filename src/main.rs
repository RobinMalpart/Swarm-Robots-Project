// src/main.rs

mod map;
mod robots;



use map::Map;
use robots::robot::Robot;

fn main() {
    // Créer une carte de taille 70x70.
    let mut map = Map::new(70, 70);

    println!("Carte initiale:");
    map.display();

    // Création d'un robot, par exemple en position (0,0) ou à côté de la base
    let robot = Robot::new(0, 0);
    map.add_robot(robot);

    // Simuler 10 tours.
    for turn in 1..=10 {
        for robot in map.robots.iter_mut() {
            // Ici, on passe la grille, la largeur et la hauteur à la méthode act().
            robot.act(&map.grid, map.width, map.height);
        }
        println!("\n--- Tour {} ---", turn);
        map.display();
    }
}
