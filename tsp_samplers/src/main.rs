mod algorithms;
mod helpers;
mod ser;

use crate::ser::save_json;
use algorithms::{hillclimb::hillclimb_steepest, snowball_sampler::SnowballSampler, PwrSampler, exhaustive_sampler::ExhaustiveSampler};
use clap::{Parser, Subcommand};
use helpers::{parse_intermediate_format, TspFile};
use std::time::{Duration, Instant};

const PBAR_W: u32 = 32;

#[derive(Parser)]
#[command(author="ur mom", version="1.0", about, long_about = None)]
struct Cli {
    input_file: String,
    iters: u32,

    #[command(subcommand)]
    algorithm: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Snowball {
        walk_len: u32,
        n_edges: u32,
        depth: u32,
        mut_d: usize,
        seed: Option<u64>,
    },
    Pwr {
        n_max: u32,
        n_att: u32,
        e_att: u32,
        seed: Option<u64>,
    },
    Exhaustive {

    }
}

fn main() {
    let cli = Cli::parse();

    let file = parse_intermediate_format(cli.input_file.as_str()).unwrap();

    println!("{:?}", file);

    match cli.algorithm {
        Commands::Snowball {
            walk_len,
            n_edges,
            depth,
            mut_d,
            seed,
        } => sample_snowball(file, walk_len, n_edges, depth, mut_d, cli.iters, seed),
        Commands::Pwr {
            n_max,
            n_att,
            e_att,
            seed,
        } => sample_pwr(file, n_max, n_att, e_att, cli.iters, seed),
        Commands::Exhaustive {  } => sample_exhaustive(file),
    }
}

fn sample_exhaustive(file: TspFile) {
    let mut sampler = ExhaustiveSampler::new(file.distance_matrix);
    sampler.sample();
    let (nodes, edges) = sampler.get_samples();

    let _ = std::fs::create_dir("exhaustive_latest");
    let path = format!("exhaustive_latest/samples.json");
    save_json(
        nodes,
        edges,
        0,
        0,
        path.as_str(),
    ).unwrap();
}

fn sample_pwr(file: TspFile, n_max: u32, n_att: u32, e_att: u32, iters: u32, seed: Option<u64>) {
    let mut pwrsampler = PwrSampler::new(file.distance_matrix, seed);

    let dt = chrono::offset::Local::now().to_string();
    let _ = std::fs::create_dir(format! {"pwr_{}", dt});
    let _ = std::fs::create_dir("pwr_latest");

    let mut time_ms = 0;

    for i in 0..iters {
        print_progress_bar(i + 1, iters, PBAR_W);
        let start = Instant::now();
        pwrsampler.sample(n_max, n_att, e_att);
        time_ms += start.elapsed().as_millis();
        let (nodes, edges) = pwrsampler.get_samples();

        let path = format!("pwr_latest/samples_{}.json", i);
        let path2 = format!("pwr_{}/samples_{}.json", dt, i);
        let paths = [path, path2];

        for path in paths {
            save_json(
                nodes,
                edges,
                pwrsampler.get_hc_calls(),
                time_ms,
                path.as_str(),
            )
            .unwrap();
        }
    }
}

fn sample_snowball(
    file: TspFile,
    walk_len: u32,
    n_edges: u32,
    depth: u32,
    mut_d: usize,
    iters: u32,
    seed: Option<u64>,
) {
    let mut snowball_sampler = SnowballSampler::new(
        walk_len,
        n_edges,
        depth,
        mut_d,
        file.distance_matrix,
        hillclimb_steepest,
        seed,
    );

    let dt = chrono::offset::Local::now().to_string();
    let _ = std::fs::create_dir(format! {"snowball_{}", dt});
    let _ = std::fs::create_dir("snowball_latest");

    let mut time_ms = 0;

    for i in 0..iters {
        print_progress_bar(i + 1, iters, PBAR_W);

        let start = Instant::now();
        snowball_sampler.sample();
        time_ms += start.elapsed().as_millis();

        let (nodes, edges) = snowball_sampler.get_samples();

        let path = format!("snowball_latest/samples_{}.json", i);
        let path2 = format!("snowball_{}/samples_{}.json", dt, i);
        let paths = [path, path2];

        for path in paths {
            save_json(
                nodes,
                edges,
                snowball_sampler.get_hc_calls(),
                time_ms,
                path.as_str(),
            )
            .unwrap();
        }
    }
}

fn print_progress_bar(i: u32, max: u32, width: u32) {
    let progress = i as f32 / max as f32;
    let filled = progress * (width as f32);
    for i in 0..width {
        if i > filled as u32 {
            print!(".");
        } else {
            print!("#");
        }
    }
    print!(" {}/{}\n", i, max);
}
