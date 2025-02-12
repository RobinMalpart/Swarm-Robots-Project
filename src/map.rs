// src/map.rs

use rand::Rng;
use noise::{NoiseFn, Perlin};
use robot::Robot;

use crate::robots::robot;

/// Represents a 2D map for robotic exploration.
#[derive(Debug)]
pub struct Map {
    pub width: usize,              // Map width
    pub height: usize,             // Map height
    pub grid: Vec<Vec<char>>,      // 2D grid storing map elements
    pub base_x: usize,             // X-coordinate of the base
    pub base_y: usize,             // Y-coordinate of the base
    pub robots: Vec<crate::Robot>, // List of robots on the map
}

impl Map {
    /// Creates a new map with the specified width and height.
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Map {
            width,
            height,
            grid: vec![vec![' '; width]; height],
            base_x: 0,
            base_y: 0,
            robots: Vec::new(),
        };

        map.generate_terrain();
        map.place_resources();
        map.place_science_base();
        map
    }

    /// Generates terrain using Perlin noise.
    fn generate_terrain(&mut self) {
        let seed: u32 = rand::thread_rng().gen_range(0..u32::MAX);
        let perlin = Perlin::new(seed);
        let scale = 10.0;
        for y in 0..self.height {
            for x in 0..self.width {
                let noise_value = perlin.get([x as f64 / scale, y as f64 / scale]);
                self.grid[y][x] = if noise_value < 0.2 { '.' } else { '#' };
            }
        }
    }

    /// Places clusters of resources.
    fn place_resources(&mut self) {
        let seed: u32 = rand::thread_rng().gen_range(0..u32::MAX);
        let perlin = Perlin::new(seed);
        let scale = 5.0;
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                let noise_value = perlin.get([x as f64 / scale, y as f64 / scale]);
                if noise_value > 0.4 && rng.gen_bool(0.1) {
                    self.grid[y][x] = '⚡'; // Energy
                } else if noise_value > 0.2 && rng.gen_bool(0.1) {
                    self.grid[y][x] = '⛏'; // Mineral
                }
            }
        }
    }

    /// Places a science base on the map.
    fn place_science_base(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            // Place the base only if the cell is normal terrain and surrounded by normal terrain.
            if self.grid[y][x] == '.' && self.is_surrounded_by_terrain(x, y) {
                self.grid[y][x] = '🏭'; // Base
                self.base_x = x;
                self.base_y = y;
                break;
            }
        }
    }

    /// Checks if the cell (x, y) is walkable (normal terrain).
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == '.'
    }

    /// Checks if the cell (x, y) is surrounded by normal terrain ('.') in all directions.
    fn is_surrounded_by_terrain(&self, x: usize, y: usize) -> bool {
        let directions = [
            (0, 1), (1, 0), (0, -1), (-1, 0),
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                if self.grid[ny as usize][nx as usize] != '.' {
                    return false;
                }
            }
        }
        true
    }

    /// Displays the map as ASCII in the terminal, superimposing robot positions.
    pub fn display(&self) {
        let mut display_grid = self.grid.clone();
        for robot in &self.robots {
            // Replace the character with the robot emoji 🤖
            display_grid[robot.y][robot.x] = '🤖';
        }
        for row in &display_grid {
            println!("{}", row.iter().collect::<String>());
        }
    }

    /// Adds a robot to the map.
    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }
}
