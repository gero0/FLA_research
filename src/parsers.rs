use crate::helpers::*;
use std::{error::Error, fs};

#[derive(Debug, Clone)]
pub struct TspFile {
    pub name: String,
    pub dimension: u32,
    pub distance_matrix: Vec<Vec<i32>>,
}

fn parse_euc_2d(lines: std::str::Lines) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut nodes = vec![];

    //parse nodes
    for (i, line) in lines.enumerate() {
        let line = line.trim();
        if line == "EOF" {
            break;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let id: u32 = tokens[0].trim().parse()?;
        let x: f32 = tokens[1].trim().parse()?;
        let y: f32 = tokens[2].trim().parse()?;
        nodes.push(Node { pos: i, id, x, y });
    }

    Ok(generate_distance_matrix(&nodes))
}

fn parse_full_matrix(lines: std::str::Lines, dim: u32) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut d_matrix = vec![];
    for (i, line) in lines.enumerate() {
        if i >= dim as usize {
            break;
        }
        let mut row = vec![];
        let line = line.trim();
        let tokens = line.split_whitespace();
        for token in tokens {
            row.push(token.parse::<i32>()?)
        }
        d_matrix.push(row);
    }

    Ok(d_matrix)
}

pub fn parse_tsp_file(path: &str) -> Result<TspFile, Box<dyn Error>> {
    let file = fs::read_to_string(path).unwrap();
    let mut lines = file.lines();
    let mut dimension = None;
    let mut name = None;
    let mut edge_wf = "EUC_2D";

    for line in lines.by_ref() {
        let line = line.trim();
        if line == "NODE_COORD_SECTION" || line == "EDGE_WEIGHT_SECTION" {
            break;
        }
        let tokens: Vec<&str> = line.split(':').collect();
        let key = tokens[0].trim();
        let val = tokens[1].trim();

        match key {
            "DIMENSION" => dimension = Some(val.parse::<u32>()?),
            "NAME" => name = Some(val),
            "EDGE_WEIGHT_FORMAT" => edge_wf = val,
            _ => {}
        }
    }

    //return error if we don't have a dimension provided
    let dimension = dimension.ok_or(std::fmt::Error)?;
    let name = String::from(name.unwrap_or(""));

    let distance_matrix = match edge_wf {
        "EUC_2D" => parse_euc_2d(lines)?,
        "FULL_MATRIX" => parse_full_matrix(lines, dimension)?,
        _ => unimplemented!(),
    };

    Ok(TspFile {
        name,
        dimension,
        distance_matrix,
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
