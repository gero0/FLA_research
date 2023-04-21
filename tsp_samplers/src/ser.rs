use std::{error::Error, fs::File, io::{Write, BufWriter}};

use crate::algorithms::{EdgeMap, NodeMap};

pub enum JsonField{
    String(&'static str),
    Int(u128),
    Float(f64),
}

pub fn save_json(
    nodes: &NodeMap,
    edges: &EdgeMap,
    hc_count: u64,
    oracle_count: u128,
    time_ms: u128,
    comment : &str,
    addl_fields: &[(&str, JsonField)],
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let nodes = nodes.into_iter();
    let edges = edges.into_iter();

    let f = File::create(path)?;
    let mut f = BufWriter::with_capacity(1024 * 1024, f);

    f.write("{\n".as_bytes())?;
    f.write_fmt(format_args!(
        "\"opt_count\":{},\n\"oracle_count\":{},\n\"time_ms\":{},\n\"comment\":\"{}\",\n",
        hc_count, oracle_count, time_ms, comment
    ))?;

    for (key, val) in addl_fields {
        f.write_fmt(format_args!("\"{}\":", key))?;
        match val {
            JsonField::Int(i) => f.write_fmt(format_args!("{},", i))?,
            JsonField::Float(i) => f.write_fmt(format_args!("{},", i))?,
            JsonField::String(s) => f.write_fmt(format_args!("\"{}\",", s))?,
        }
        f.write("\n".as_bytes())?;
    }

    f.write("\"nodes\": [\n".as_bytes())?;
    for (i, node) in nodes.enumerate() {
        let (perm, (id, len)) = node;
        let pref = match i {
            0 => "",
            _ => ",\n",
        };
        f.write_fmt(format_args!("{}[{},{:?},{}]", pref, id, perm, len))?;
    }
    f.write("],\n".as_bytes())?;
    f.write("\"edges\": [\n".as_bytes())?;
    for (i, edge) in edges.enumerate() {
        let ((src, dst), len) = edge;
        let pref = match i {
            0 => "",
            _ => ",\n",
        };
        f.write_fmt(format_args!("{}[{},{},{}]", pref, src, dst, len))?;
    }
    f.write("]\n".as_bytes())?;
    f.write("}".as_bytes())?;
    Ok(())
}
