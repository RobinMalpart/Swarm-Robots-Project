// src/robots/robot.rs

use crate::map::Map;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
}

impl Robot {
    /// Crée un nouveau robot à la position (x, y).
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Le robot se déplace d'une case par tour dans une direction aléatoire
    /// parmi les cellules accessibles (walkable). Le robot ne peut pas aller
    /// sur un caillou (cellule contenant '#').
    pub fn act(&mut self, grid: &Vec<Vec<char>>, width: usize, height: usize) {
        // Liste des déplacements possibles (dx, dy)
        let directions = [
            (0isize, -1isize), // Haut
            (0, 1),           // Bas
            (-1, 0),          // Gauche
            (1, 0),           // Droite
        ];

        // Filtrer les directions valides
        let valid_moves: Vec<(usize, usize)> = directions
            .iter()
            .filter_map(|(dx, dy)| {
                let new_x = self.x as isize + dx;
                let new_y = self.y as isize + dy;
                if new_x >= 0
                    && new_x < width as isize
                    && new_y >= 0
                    && new_y < height as isize
                    && grid[new_y as usize][new_x as usize] == '.'
                {
                    Some((new_x as usize, new_y as usize))
                } else {
                    None
                }
            })
            .collect();

        // Si des déplacements valides existent, en choisir un au hasard
        if let Some(&(new_x, new_y)) = valid_moves.choose(&mut thread_rng()) {
            self.x = new_x;
            self.y = new_y;
        }
    }
}