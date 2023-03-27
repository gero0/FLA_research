import argparse

from igraph import Graph
import igraph
from helpers import split_edge_data
from main import process_file


parser = argparse.ArgumentParser(prog="graph_vis",
                                    description="What the program does")
parser.add_argument("filename",
                    help="Name of input file")

parser.add_argument("-o",
                    "--output",
                    help="Name of output file",
                    required=False)

args = parser.parse_args()

if args.output is None:
    output = "graphvis.pdf"
else:
    output = args.output

nodes, edges, time, hc_count, oracle_count = process_file(args.filename)

(edge_list, weight_list) = split_edge_data(edges)
g = Graph(n=len(nodes),
            edges=edge_list,
            edge_attrs={"weight": weight_list},
            directed=False)

igraph.plot(g, target = output)