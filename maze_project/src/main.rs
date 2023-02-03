// Marija Petrovic, E2 88/2022

use std::{fs::File, io::Read};
use std::{io, char};

#[derive(Debug, Clone)]
struct Node {
    id: i32,
    west: Option<Box<Node>>,
    east: Option<Box<Node>>,
    north: Option<Box<Node>>,
    south: Option<Box<Node>>,
    has_doors: [bool; 4],
    has_key: bool,
    is_end: bool,
}

impl Node {
    fn new(id: i32, doors: [bool; 4], key: bool, exit: bool) -> Self {
        Self { id, has_doors: doors, has_key: key, is_end: exit, west: None, east: None, north: None, south: None }
    }
}

fn main() {
    let file_content = read_file().unwrap();
    let cells: Vec<&str> = file_content.split('\n').collect();

    let mut maze = create_maze(&cells);
    let mut player:(Vec<(i32, i32)>, i32) = (Vec::new(), 0);
    let mut final_path: Vec<(i32, i32)> = Vec::new();
    find_shortest_path(&mut maze, &mut player, 0,  &mut final_path);
    print!("{:?}", final_path);
}

fn read_file() -> Result<String, io::Error> {
    let mut from_file = String::new();
    File::open("maze.txt")?.read_to_string(&mut from_file)?;
    Ok(from_file)
}

fn create_maze(cells: &Vec<&str>) -> Vec<Node> {
    let mut maze: Vec<Node> = Vec::new();
    let mut index = 0;
    for cell in cells {
        let split_cell: Vec<&str> = cell.split_whitespace().collect();
        let mut doors:[bool; 4] = [false, false, false, false];
        let mut door_index = 0;
        for c in split_cell.get(1).unwrap().chars() {
            if c == '0' {
                doors[door_index] = false;
            } else {
                doors[door_index] = true;
            }
            door_index += 1;
        }

        let mut key = false;
        let mut exit = false;
        let mut key_and_exit_index = 0;
        let mut key_exit_bits: [char; 4] = [' ', ' ', ' ', ' '];
        for c in split_cell.get(2).unwrap().chars() {
            key_exit_bits[key_and_exit_index] = c;
            key_and_exit_index += 1;
        }

        if key_exit_bits[0] == '1' && key_exit_bits[1] == '1' {
            key = true;
        }

        if key_exit_bits[2] == '1' && key_exit_bits[3] == '1' {
            exit = true;
        }

        maze.push(Node::new(index, doors, key, exit));
        index += 1;
    }

    let cloned_maze = maze.clone();
    for mut node in &mut maze {
        let cell = cells.get(node.clone().id as usize).unwrap();
        let split_cells: Vec<&str> = cell.split_whitespace().collect();
        let mut index = 0;
        for c in split_cells.get(0).unwrap().chars() {
            if c == '1' {
                match index {
                    0 => {
                        let id = node.id;
                        node.west = cloned_maze.iter().find(|node| node.id == id-1).map(|node| Box::new(node.clone()));
                    },
                    1 => {
                        let id = node.id;
                        node.east = cloned_maze.iter().find(|node| node.id == id+1).map(|node| Box::new(node.clone()));
                    },
                    2 => {
                        let id = node.id;
                        node.north = cloned_maze.iter().find(|node| node.id == id-9).map(|node| Box::new(node.clone()));
                    },
                    3 => {
                        let id = node.id;
                        node.south = cloned_maze.iter().find(|node| node.id == id+9).map(|node| Box::new(node.clone()));
                    },
                    _ => {}
                }
            }
            index += 1;
        }
    }

    return maze;
}

fn find_shortest_path(maze: &mut Vec<Node>, player: &mut (Vec<(i32, i32)>, i32), node_id: i32, finish_path: &mut Vec<(i32, i32)>) {
    let current_node = maze.iter().find(|node| node.id == node_id).cloned().unwrap();

    if finish_path.len() > 0 {
        return;
    }

    if current_node.has_key == true && !player.0.iter().any(|&node| node.0 == current_node.id) {
        player.1 += 1;
    }

    if !player.0.iter().any(|&node| node == (current_node.id, player.1)) {
        player.0.push((current_node.id, player.1));
    } else {
        return;
    }

    if current_node.is_end {
        *finish_path = player.0.clone();
        return;
    }

    if let Some(node) = current_node.south {
        if !current_node.has_doors[3] {
            find_shortest_path(maze, player, node.id, finish_path);
        } else {
            if player.1 > 0 {
                player.1 -= 1;
                open_door(maze, 3, current_node.id);
                open_door(maze, 2, node.id);
                find_shortest_path(maze, player, node.id, finish_path);
            }
        }
    }

    if let Some(node) = current_node.east {
        if !current_node.has_doors[0] {
            find_shortest_path(maze, player, node.id, finish_path);
        } else {
            if player.1 > 0 {
                player.1 -= 1;
                open_door(maze, 1, current_node.id);
                open_door(maze, 0, node.id);
                find_shortest_path(maze, player, node.id, finish_path);
            }
        }
    }
    
    if let Some(node) = current_node.north {
        if !current_node.has_doors[2] {
            find_shortest_path(maze, player, node.id, finish_path);
        } else {
            if player.1 > 0 {
                player.1 -= 1;
                open_door(maze, 2, current_node.id);
                open_door(maze, 3, node.id);
                find_shortest_path(maze, player, node.id, finish_path);
            }
        }
    }

    if let Some(node) = current_node.west {
        if !current_node.has_doors[0] {
            find_shortest_path(maze, player, node.id, finish_path);
        } else {
            if player.1 > 0 {
                player.1 -= 1;
                open_door(maze, 0, current_node.id);
                open_door(maze, 1, node.id);
                find_shortest_path(maze, player, node.id, finish_path);
            }
        }
    }
}

fn open_door(maze: &mut Vec<Node>, side: usize, id: i32) {
    if let Some(node) = maze.iter_mut().find(|node| node.id == id) {
        node.has_doors[side] = false;
    }
}