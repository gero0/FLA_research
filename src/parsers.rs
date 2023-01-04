use crate::helpers::*;
use std::{error::Error, fs};

#[derive(Debug, Clone)]
pub struct TspFile {
    pub name: String,
    pub dimension: u32,
    pub nodes: Vec<Node>,
}

pub fn parse_tsp_file(path: &str) -> Result<TspFile, Box<dyn Error>> {
    let file = fs::read_to_string(path).unwrap();
    let mut lines = file.lines();
    let mut dimension = None;
    let mut name = None;

    for line in lines.by_ref() {
        let line = line.trim();
        if line == "NODE_COORD_SECTION" {
            break;
        }
        let tokens: Vec<&str> = line.split(':').collect();
        let key = tokens[0].trim();
        let val = tokens[1].trim();

        match key {
            "DIMENSION" => dimension = Some(val.parse::<u32>()?),
            "NAME" => name = Some(val),
            _ => {}
        }
    }

    //return error if we don't have a dimension provided
    let dimension = dimension.ok_or(std::fmt::Error)?;
    let name = String::from(name.unwrap_or(""));

    let mut nodes = vec![];

    //parse nodes
    for line in lines {
        let line = line.trim();
        if line == "EOF" {
            break;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let id: u32 = tokens[0].trim().parse()?;
        let x: f32 = tokens[1].trim().parse()?;
        let y: f32 = tokens[2].trim().parse()?;
        nodes.push(Node { id: id-1, x, y });
    }

    Ok(TspFile {
        name,
        dimension,
        nodes,
    })
}

pub fn parse_tour_file(path: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let file = fs::read_to_string(path).unwrap();
    let mut lines = file.lines();

    for line in lines.by_ref() {
        let line = line.trim();
        if line == "TOUR_SECTION" {
            break;
        }
    }

    let mut path = vec![];

    //parse path
    for line in lines {
        let line = line.trim();
        if line == "-1" {
            break;
        }
        let id: u32 = line.parse()?;
        path.push(id - 1);
    }

    Ok(path)
}
