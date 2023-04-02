from math import sqrt
import pathlib
from PIL import Image, ImageDraw


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


def write_points(points, fname):
    with open(fname, 'w+') as f:
        for i, point in enumerate(points):
            f.write(f"{i} {point[0]} {point[1]}\n")


def save_res(dirname, points, max_x, max_y, name, border=10, radius=5):
    pathlib.Path(dirname).mkdir(parents=True, exist_ok=True)

    matrix = calc_dist_matrix(points)
    write_matrix(matrix, f"{dirname}/matrix.txt", name)
    write_points(matrix, f"{dirname}/points.txt")

    img = Image.new(mode="RGB", size=(max_x + 2 * border, max_y + 2 * border))
    draw = ImageDraw.Draw(img)

    for point in points:
        (x, y) = point
        (draw_x, draw_y) = (x + border, y + border)
        draw.ellipse((draw_x - radius, draw_y - radius, draw_x + radius,
                      draw_y + radius),
                     fill=(255, 255, 255),
                     outline=(255, 0, 0))

    img.save(f"{dirname}/vis.png")