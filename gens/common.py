from math import sqrt
import pathlib
from PIL import Image


def find_extremes(points):
    min_x, max_x, min_y, max_y = 0, 0, 0, 0

    for point in points:
        if point[0] < min_x:
            min_x = point[0]

        if point[0] > max_x:
            max_x = point[0]

        if point[1] < min_y:
            min_y = point[1]

        if point[1] > max_y:
            max_y = point[1]

    return min_x, max_x, min_y, max_y


def calc_dist_matrix(points):
    matrix = []
    for x in range(0, len(points)):
        row = []
        for y in range(0, len(points)):
            if x == y:
                row.append(0)
            else:
                p1 = points[x]
                p2 = points[y]
                dist = sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2)
                row.append(round(dist))

        matrix.append(row)

    return matrix

def write_matrix(matrix, fname):
    with open(fname, 'w+') as f:
        for row in matrix:
            for element in row:
                f.write(f"{element} ")
            f.write("\n")


def write_points(points, fname):
    with open(fname, 'w+') as f:
        for i, point in enumerate(points):
            f.write(f"{i} {point[0]} {point[1]}\n")

def save_res(dirname, points, max_x, max_y):
    pathlib.Path(dirname).mkdir(parents=True, exist_ok=True)

    matrix = calc_dist_matrix(points)
    write_matrix(matrix, f"{dirname}/matrix.txt")
    write_points(matrix, f"{dirname}/points.txt")

    img = Image.new(mode="RGB", size=(max_x, max_y))

    for point in points:
        img.putpixel(point, (255, 255, 255))

    img.save(f"{dirname}/vis.png")