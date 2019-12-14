#![feature(entry_insert)]

use crate::data::{Memory, Program};
use itertools::Itertools;
use std::collections::BTreeMap;

mod data;

fn main() {
    let memory: Memory = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(memory.clone()));
    println!("part_2: {}", part_2(memory.clone()));
}

// Every three output instructions specify the x position (distance from the left), y position
// (distance from the top), and tile id. The tile id is interpreted as follows:
// - 0 is an empty tile. No game object appears in this tile.
// - 1 is a wall tile. Walls are indestructible barriers.
// - 2 is a block tile. Blocks can be broken by the ball.
// - 3 is a horizontal paddle tile. The paddle is indestructible.
// - 4 is a ball tile. The ball moves diagonally and bounces off objects.
// How many block tiles are on the screen when the game exits
fn part_1(memory: Memory) -> usize {
    let mut program = Program::new(memory, None);
    program.run();

    let mut block_tile_count = 0;

    for mut chunk in &program.output.clone().into_iter().chunks(3) {
        let tile = chunk.nth(2).unwrap();
        if tile == 2 {
            block_tile_count += 1;
        }
    }

    block_tile_count
}

// Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play
// for free.  When three output instructions specify X=-1, Y=0, the third output instruction is not
// a tile; the value instead specifies the new score to show in the segment display. For example, a
// sequence of output values like -1,0,12345 would show 12345 as the player's current score.
// Beat the game by breaking all the blocks. What is your score after the last block is broken?
fn part_2(memory: Memory) -> isize {
    let mut program = Program::new(memory, None);

    // Insert to quarters
    program.memory[0] = 2;

    // Instead of just one paddle tile at the bottom, make the entire bottom a paddle
    for paddle_idx in 1520..=1557 {
        program.memory[paddle_idx] = 3;
    }

    // Run the game
    loop {
        program.run();

        // Uncomment to draw the state of the game! For example:
        //
        // Score: 5944
        // wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
        // w______________________________________w
        // w___b__bbbb_b__b__bbbb_____b_b_bbb_b___w
        // w_bbb_bb__b_bbbbbb__bb___bb__bb_bbbb_b_w
        // w__bb_bbbb_bbbb_b_bb__bbb_bbbbbb__bb___w
        // w_bbbb_bbbb_bbbb_bb___b_____b__bb_b__b_w
        // w_bbbbb_b_b_b_bbbb_b_bbb_bbbbbb_bbb_b__w
        // w__b___bb_b__b_bb_____bbb__b_bb_bb_____w
        // w______bbbbbbbbbb_b__bbbbb_bbb__b___b__w
        // w_______b_b_____bbbbbbb__b__b_bbbbb_bb_w
        // w__b________bbb_b_b_______b______bbbbb_w
        // w____________b__b_b___b___________bbb__w
        // w____________b__bb_______________bbbbb_w
        // w_______________________b_______bb_____w
        // w__b___________________bb______b_b___b_w
        // w____b____________b_____b______b_______w
        // w__b__b________________________________w
        // w_____b________________________________w
        // w______________________________________w
        // w______________________________________w
        // w______________________________________w
        // w_____o________________________________w
        // wppppppppppppppppppppppppppppppppppppppw
        // w______________________________________w
        //
        // draw(&program);

        // Game is over; return the score
        if program.exit_code == Some(99) {
            return *program.output.last().unwrap();
        }

        // Just tell the "paddle" to "stay put"
        program.input.push(0);
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
// y, x
struct Coord(isize, isize);

#[derive(Debug)]
struct Tile(isize);

impl Tile {
    pub fn display(&self) -> char {
        match self.0 {
            0 => '_',
            1 => 'w',
            2 => 'b',
            3 => 'p',
            4 => 'o',
            _ => panic!("Unknown tile: {}", self.0),
        }
    }
}

fn draw(program: &Program) {
    let mut to_draw: BTreeMap<Coord, Tile> = BTreeMap::new();

    for mut chunk in &program.output.clone().into_iter().chunks(3) {
        let x = chunk.nth(0).unwrap();
        let y = chunk.nth(0).unwrap();
        let tile = chunk.nth(0).unwrap();
        to_draw.insert(Coord(y, x), Tile(tile));
    }

    let mut to_draw_iter = to_draw.iter().peekable();

    while let Some((coord, tile)) = to_draw_iter.next() {
        // Draw score tile
        if coord.1 == -1 {
            println!("Score: {}", tile.0);
            continue;
        }

        // Render normal tile
        print!("{}", tile.display());

        // Render line break or screen break
        if let Some((next_coord, _next_tile)) = to_draw_iter.peek() {
            if coord.0 != next_coord.0 {
                print!("\n");
            }
        } else {
            println!("\n\n\n\n\n\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}
