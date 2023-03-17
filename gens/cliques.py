from math import sin, cos, pi, sqrt
import numpy as np
import argparse
import pathlib
from common import *
from PIL import Image

from common import find_extremes

parser = argparse.ArgumentParser(
    prog="clique tsp gen", description="generates cities forming cliques")

parser.add_argument(
    "N",
    help="Number of cities",
)

parser.add_argument(
    "N_cliques",
    help="Number of cliques",
)

parser.add_argument("-minld",
                    help="Minimum distance between cities in clique",
                    required=False)

parser.add_argument("-maxld",
                    help="Maximum distance between cities in clique",
                    required=False)

parser.add_argument("-mincd",
                    help="Minimum distance between clique centers",
                    required=False)

parser.add_argument("-maxcd",
                    help="Maximum distance between clique centers",
                    required=False)

parser.add_argument("-o",
                    "--output",
                    help="Name of output directory",
                    required=False)

args = parser.parse_args()

N = int(args.N)
N_cliques = int(args.N_cliques)

if args.minld is None:
    MinLocalSep = 5
else:
    MinLocalSep = int(args.minld)

if args.maxld is None:
    MaxLocalSep = 30
else:
    MaxLocalSep = int(args.maxld)

if args.maxcd is None:
    MaxCliqueSep = 300
else:
    MaxCliqueSep = int(args.maxcd)

if args.mincd is None:
    MinCliqueSep = 100
else:
    MinCliqueSep = int(args.mincd)

ppc = int(N / N_cliques)

rng = np.random.default_rng()

total_point_counter = 0
MAX_RETRIES = 1000


def to_cartesian(r, angle):
    x = int(r * cos(angle))
    y = int(r * sin(angle))

    return x, y


#Starting clique - coordinates (0,0)
clique_centers = [(0, 0)]

while (len(clique_centers) != N_cliques):
    # Find a random point around starting clique
    r = rng.uniform(MinCliqueSep, MaxCliqueSep)
    angle = rng.uniform(0, 2 * pi)
    x, y = to_cartesian(r, angle)

    ok = True

    #Check if it's far enough from existing cliques, if not, try again
    for clique in clique_centers:
        dist = sqrt((x - clique[0])**2 + (y - clique[1])**2)
        if (dist < MinCliqueSep):
            ok = False
            break

    if not ok:
        continue

    clique_centers.append((x, y))

points = []

for clique in clique_centers:
    for i in range(0, ppc):
        if (total_point_counter < N):
            retries = 0
            while(retries < MAX_RETRIES):
                r = rng.uniform(MinLocalSep, MaxLocalSep)
                angle = rng.uniform(0, 2 * pi)
                x, y = to_cartesian(r, angle)

                c_x, c_y = clique
                p_x = c_x + x
                p_y = c_y + y

                ok = True

                for point in points:
                    dist = sqrt((p_x - point[0])**2 + (p_y - point[1])**2)
                    if (dist < MinLocalSep):
                        ok = False
                        break

                if not ok:
                    retries += 1
                    continue

                total_point_counter += 1
                points.append((p_x, p_y))
                break

            if retries >= MAX_RETRIES:
                print("ERROR: Could not generate problem with requested parameters, try decreasing min local sep or decreasing number of points")
                exit()

#Move point coordinates to start at 0
min_x, max_x, min_y, max_y = find_extremes(points)

remapped = []

for point in points:
    remapped.append((point[0] + abs(min_x), point[1] + abs(min_y)))

print(remapped)

dirname = args.output
if dirname is None:
    dirname = "cliques_output"

pathlib.Path(dirname).mkdir(parents=True, exist_ok=True)

save_res(dirname, remapped, max_x + abs(min_x) + 1, max_y + abs(min_y) + 1,
         f"cliques_{N}")
