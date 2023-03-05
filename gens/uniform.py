import numpy as np
from PIL import Image
import argparse
import pathlib
from common import *

parser = argparse.ArgumentParser(
    prog="Uniform tsp gen",
    description="generates uniformly distributed cities")

parser.add_argument(
    "N",
    help="Number of cities",
)

parser.add_argument(
    "area_size",
    help=
    "Length of side of square area on which cities will be randomly distributed",
)

parser.add_argument("-o",
                    "--output",
                    help="Name of output directory",
                    required=False)

args = parser.parse_args()

N = int(args.N)
MaxD = int(args.area_size)

rng = np.random.default_rng()

points = []

for i in range(0, N):
    x = rng.uniform(0, 1) * MaxD
    y = rng.uniform(0, 1) * MaxD
    points.append((int(x), int(y)))

dirname = args.output
if dirname is None:
    dirname = "uniform_output"

save_res(dirname, points, MaxD, MaxD, f"uniform_{N}")
