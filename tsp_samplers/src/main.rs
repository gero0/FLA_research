mod algorithms;
mod helpers;
mod ser;

use crate::ser::save_json;
use algorithms::{
    exhaustive_sampler::ExhaustiveSampler, hillclimb::hillclimb_steepest,
    snowball_sampler::SnowballSampler, PwrSampler, SamplingAlg,
};
use clap::{Parser, Subcommand};
use helpers::{parse_intermediate_format, TspFile};
use std::time::Instant;

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
    Exhaustive {},
}

fn save_sampling_results(
    sampler: &impl SamplingAlg,
    time_ms: u128,
    dirname: &str,
    dt: &str,
    i: u32,
) {
    let dir = format!("{}_{}", dirname, dt);
    let dir_latest = format!("{}_latest", dirname);

    let _ = std::fs::create_dir(&dir);
    let _ = std::fs::create_dir(&dir_latest);
    let path = format!("{}/samples_{}.json", &dir, i);
    let path2 = format!("{}/samples_{}.json", &dir_latest, i);

    let (nodes, edges) = sampler.get_samples();
    let hc_c = sampler.get_hc_calls();

    save_json(nodes, edges, hc_c, time_ms, path.as_str()).unwrap();
    save_json(nodes, edges, hc_c, time_ms, path2.as_str()).unwrap();
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
        Commands::Exhaustive {} => sample_exhaustive(file),
    }
}

fn sample_exhaustive(file: TspFile) {
    let mut sampler = ExhaustiveSampler::new(file.distance_matrix, 2, hillclimb_steepest);
    let start = Instant::now();
    sampler.sample();
    let time_ms = start.elapsed().as_millis();
    let dt = chrono::offset::Local::now().to_string();

    save_sampling_results(&sampler, time_ms, "exhaustive", &dt, 0);
}

fn sample_pwr(file: TspFile, n_max: u32, n_att: u32, e_att: u32, iters: u32, seed: Option<u64>) {
    let mut sampler = PwrSampler::new(file.distance_matrix, seed);
    let dt = chrono::offset::Local::now().to_string();
    let mut time_ms = 0;

    for i in 0..iters {
        print_progress_bar(i + 1, iters, PBAR_W);
        let start = Instant::now();
        sampler.sample(n_max, n_att, e_att);
        time_ms += start.elapsed().as_millis();
        save_sampling_results(&sampler, time_ms, "pwr", &dt, i);
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
    let mut sampler = SnowballSampler::new(
        walk_len,
        n_edges,
        depth,
        mut_d,
        file.distance_matrix,
        hillclimb_steepest,
        seed,
    );

    let dt = chrono::offset::Local::now().to_string();
    let mut time_ms = 0;

    for i in 0..iters {
        print_progress_bar(i + 1, iters, PBAR_W);

        let start = Instant::now();
        sampler.sample();
        time_ms += start.elapsed().as_millis();

        save_sampling_results(&sampler, time_ms, "snowball", &dt, i)
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
