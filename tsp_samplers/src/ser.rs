use std::{error::Error, fs::File, io::Write};

use crate::algorithms::{EdgeMap, NodeMap};

pub fn save_json(
    nodes: &NodeMap,
    edges: &EdgeMap,
    hc_count: u64,
    time_ms: u64,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let nodes = nodes.into_iter();
    let edges = edges.into_iter();

    let mut f = File::create(path)?;
    f.write("{\n".as_bytes())?;
    f.write_fmt(format_args!(
        "\"hc_count\":{},\n\"time_ms\":{},\n",
        hc_count, time_ms
    ))?;
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
