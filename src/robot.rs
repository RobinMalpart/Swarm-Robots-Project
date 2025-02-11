use crate::map::Map;

/// Trait defining explorer behavior.
pub trait Explorer {
    /// Explores the surroundings and returns the coordinates of a discovered mineral resource (if any).
    ///
    /// # Arguments
    ///
    /// * `robot` - A mutable reference to the robot performing exploration.
    /// * `map` - A reference to the map.
    ///
    /// # Returns
    ///
    /// An option containing the coordinates of a discovered mineral resource.
    fn explore(&mut self, robot: &mut Robot, map: &Map) -> Option<(usize, usize)>;
}

/// Trait defining miner behavior.
pub trait Miner {
    /// Moves the robot to a target resource location and mines it.
    ///
    /// # Arguments
    ///
    /// * `robot` - A mutable reference to the robot.
    /// * `map` - A mutable reference to the map.
    /// * `target` - The coordinates of the resource to mine.
    fn mine(&mut self, robot: &mut Robot, map: &mut Map, target: (usize, usize));
}

/// Trait defining scientist behavior.
pub trait Scientist {
    /// Searches for energy resources and returns the coordinates if one is found.
    ///
    /// # Arguments
    ///
    /// * `robot` - A mutable reference to the robot.
    /// * `map` - A mutable reference to the map.
    ///
    /// # Returns
    ///
    /// An option containing the coordinates of a discovered energy resource.
    fn collect_energy(&mut self, robot: &mut Robot, map: &mut Map) -> Option<(usize, usize)>;
}

/// Default implementation for the Explorer behavior.
pub struct DefaultExplorer;

impl Explorer for DefaultExplorer {
    fn explore(&mut self, robot: &mut Robot, map: &Map) -> Option<(usize, usize)> {
        // Check adjacent cells for a mineral resource ('M').
        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_x = (robot.x as isize + dx).clamp(0, map.width as isize - 1) as usize;
                let new_y = (robot.y as isize + dy).clamp(0, map.height as isize - 1) as usize;
                if map.grid[new_y][new_x] == 'M' {
                    println!("Explorer found mineral at ({}, {})", new_x, new_y);
                    return Some((new_x, new_y));
                }
            }
        }
        // If no resource is found, move one step to the right.
        if robot.x + 1 < map.width {
            robot.x += 1;
        }
        None
    }
}

/// Default implementation for the Miner behavior.
pub struct DefaultMiner;

impl Miner for DefaultMiner {
    fn mine(&mut self, robot: &mut Robot, map: &mut Map, target: (usize, usize)) {
        // Move the robot directly to the target resource.
        robot.x = target.0;
        robot.y = target.1;
        // If the resource is present, mine it.
        if map.grid[target.1][target.0] == 'M' {
            map.grid[target.1][target.0] = '.';
            println!("Miner mined resource at ({}, {})", target.0, target.1);
        }
        // Return to base (assumed to be at (0, 0) for simplicity).
        robot.x = map.base_x;
        robot.y = map.base_y;
    }
}

/// Default implementation for the Scientist behavior.
pub struct DefaultScientist;

impl Scientist for DefaultScientist {
    fn collect_energy(&mut self, robot: &mut Robot, map: &mut Map) -> Option<(usize, usize)> {
        // Check adjacent cells for an energy resource ('e').
        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_x = (robot.x as isize + dx).clamp(0, map.width as isize - 1) as usize;
                let new_y = (robot.y as isize + dy).clamp(0, map.height as isize - 1) as usize;
                if map.grid[new_y][new_x] == 'e' {
                    println!("Scientist found energy at ({}, {})", new_x, new_y);
                    return Some((new_x, new_y));
                }
            }
        }
        // If no energy is found, move one step to the right.
        if robot.x + 1 < map.width {
            robot.x += 1;
        }
        None
    }
}

/// Represents an autonomous robot with fixed roles assigned at creation.
/// The robot's behavior is composed by providing implementations for the
/// Explorer, Miner, and Scientist traits. Each behavior is optional, allowing
/// the creation of hybrid robots.
pub struct Robot {
    /// The robot's current x-coordinate.
    pub x: usize,
    /// The robot's current y-coordinate.
    pub y: usize,
    /// The robot's energy level.
    pub energy: u32,
    /// Optional explorer behavior.
    pub explorer: Option<Box<dyn Explorer>>,
    /// Optional miner behavior.
    pub miner: Option<Box<dyn Miner>>,
    /// Optional scientist behavior.
    pub scientist: Option<Box<dyn Scientist>>,
}

impl Robot {
    /// Creates a new robot with the specified behaviors.
    ///
    /// # Arguments
    ///
    /// * `x` - Initial x-coordinate.
    /// * `y` - Initial y-coordinate.
    /// * `energy` - Starting energy level.
    /// * `explorer` - Optional explorer behavior implementation.
    /// * `miner` - Optional miner behavior implementation.
    /// * `scientist` - Optional scientist behavior implementation.
    ///
    /// # Returns
    ///
    /// A new `Robot` instance.
    pub fn new(
        x: usize,
        y: usize,
        energy: u32,
        explorer: Option<Box<dyn Explorer>>,
        miner: Option<Box<dyn Miner>>,
        scientist: Option<Box<dyn Scientist>>,
    ) -> Self {
        Self {
            x,
            y,
            energy,
            explorer,
            miner,
            scientist,
        }
    }
}
