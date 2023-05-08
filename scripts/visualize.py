import argparse

from igraph import Graph
import igraph
from helpers import split_edge_data
from main import process_file
import matplotlib.pyplot as plt

fig = plt.figure(figsize=(100, 100))
ax = fig.subplots()

parser = argparse.ArgumentParser(prog="graph_vis",
                                 description="Visualise LON graph from samples")
parser.add_argument("filename", help="Name of input file")

parser.add_argument("-o",
                    "--output",
                    help="Name of output file",
                    required=False)

args = parser.parse_args()

if args.output is None:
    output = "graphvis.png"
else:
    output = args.output

nodes, edges, time, opt_count, oracle_count = process_file(args.filename)

(edge_list, weight_list) = split_edge_data(edges)
g = Graph(n=len(nodes),
          edges=edge_list,
          edge_attrs={"weight": weight_list},
          directed=False)

# igraph.plot(g, target = output)
igraph.plot(g, target=ax, vertex_size=0.1)
fig.savefig(output)