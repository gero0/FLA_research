from math import sqrt
import argparse
import pathlib
from common import *
from PIL import Image

parser = argparse.ArgumentParser(
    prog="Grid tsp gen",
    description="generates cities in grid")

parser.add_argument(
    "N",
    help="Number of cities",
)

parser.add_argument(
    "gap",
    help=
    "grid gap between cities",
)

parser.add_argument("-o",
                    "--output",
                    help="Name of output directory",
                    required=False)

args = parser.parse_args()

N = int(args.N)
distance = int(args.gap)

w = int(sqrt(N))

i = 0

points = []

while i < N:
    x = (i % w) * distance
    y = int(i / w) * distance
    i += 1
    points.append((int(x), int(y)))


max_x, max_y = 0, 0

for point in points:
    if point[0] > max_x:
        max_x = point[0]
    
    if point[1] > max_y:
        max_y = point[1]


dirname = args.output
if dirname is None:
    dirname = "grid_output"

pathlib.Path(dirname).mkdir(parents=True, exist_ok=True)

matrix = calc_dist_matrix(points)
write_matrix(matrix, f"{dirname}/matrix.txt")
write_points(matrix, f"{dirname}/points.txt")

img  = Image.new( mode = "RGB", size = (max_x+1, max_y+1) )

for point in points:
    img.putpixel(point, (255,255,255))

img.save(f"{dirname}/vis.png")


