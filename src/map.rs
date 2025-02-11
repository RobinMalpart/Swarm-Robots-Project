use rand::Rng;
use noise::{NoiseFn, Perlin};

/// Represents a 2D map for robotic exploration.
#[derive(Debug)]
pub struct Map {
    width: usize,   // Map width
    height: usize,  // Map height
    grid: Vec<Vec<char>>, // 2D grid storing map elements
}

impl Map {
    /// Creates a new map with the specified width and height.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the map.
    /// * `height` - Height of the map.
    ///
    /// # Returns
    ///
    /// A `Map` instance with randomly generated terrain.
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Map {
            width,
            height,
            grid: vec![vec![' '; width]; height], // Initialize empty map
        };

        map.generate_terrain();
        map.place_resources();
        map.place_science_base();
        map
    }

    /// Generates terrain using Perlin noise to create natural-looking clusters.
    fn generate_terrain(&mut self) {
        let seed: u32 = rand::thread_rng().gen_range(0..u32::MAX); // Random seed for Perlin noise
        let perlin = Perlin::new(seed);
        let scale = 10.0; // Controls terrain roughness

        for y in 0..self.height {
            for x in 0..self.width {
                let noise_value = perlin.get([x as f64 / scale, y as f64 / scale]);

                self.grid[y][x] = if noise_value < 0.2 {
                    '.' // Normal terrain
                } else {
                    '#' // Obstacles (rocks or mountains)
                };
            }
        }
    }

    /// Places clusters of resources using Perlin noise.
    fn place_resources(&mut self) {
        let seed: u32 = rand::thread_rng().gen_range(0..u32::MAX); // Random seed for Perlin noise
        let perlin = Perlin::new(seed);
        let scale = 5.0; // Controls clustering of resources
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

    fn place_science_base(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            if self.grid[y][x] == '.' && self.is_surrounded_by_terrain(x, y) {
                self.grid[y][x] = '🏭'; // Base
                break;
            }
        }
    }

    /// Checks if the given position is surrounded by normal terrain, including diagonals.
    fn is_surrounded_by_terrain(&self, x: usize, y: usize) -> bool {
        let directions = [
            (0, 1),   // Down
            (1, 0),   // Right
            (0, -1),  // Up
            (-1, 0),  // Left
            (1, 1),   // Down-Right
            (1, -1),  // Up-Right
            (-1, 1),  // Down-Left
            (-1, -1), // Up-Left
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

    /// Displays the map as ASCII in the terminal.
    pub fn display(&self) {
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
