from asyncore import write
import tsplib95 as tsp
import sys
import argparse


def create_distance_matrix(problem):
    n = problem.dimension
    matrix = []
    for row_i in range(1, n + 1):
        row = []
        for col_i in range(1, n + 1):
            row.append(problem.get_weight(row_i, col_i))
        matrix.append(row)

    return matrix


def write_matrix(matrix, fname, name):
    with open(fname, 'w+') as f:
        f.write(f"{name}\n")
        f.write(f"{len(matrix)}\n")
        for y, row in enumerate(matrix):
            for x, element in enumerate(row):
                if y == x:
                    f.write("0 ")
                else:
                    f.write(f"{element} ")
            f.write("\n")


parser = argparse.ArgumentParser(
    prog="tspconvert",
    description=
    "Converts tsplib files to simplified format with full-matrix representation"
)

parser.add_argument(
    "input",
    help="Name of input file",
)

parser.add_argument("-o",
                    "--output",
                    help="Name of output file",
                    required=False)

args = parser.parse_args()

problem = tsp.load(args.input)
matrix = create_distance_matrix(problem)

output = args.output
if args.output is None:
    output = f"conv_{problem.name}.txt"

write_matrix(matrix, output, problem.name)