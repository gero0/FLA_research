from matplotlib import pyplot as plt
import pandas as pd
from helpers import all_stats
from pathlib import Path
import argparse
import os


def main():
    parser = argparse.ArgumentParser(prog="lonstats",
                                     description="What the program does")
    parser.add_argument("filename", help="Name of file containing input data")
    parser.add_argument("-s",
                        "--stats",
                        nargs="+",
                        help="Stats to calculate. Leave empty to plot all",
                        required=False)
    parser.add_argument("-o",
                        "--output",
                        help="Name of output dir",
                        required=False)

    args = parser.parse_args()

    path = args.filename
    stats = args.stats
    output = args.output

    if output is None:
        output = "plots"

    if stats is None:
        stats = all_stats

    Path(output).mkdir(exist_ok=True)

    df = pd.read_csv(path, sep=';', index_col=0)
    print(df)

    for stat in stats:
        try:
            df.plot(backend="matplotlib", x="hc_count", y=stat)
            plt.savefig(os.path.join(output, stat))
        except:
            print(f"Stat {stat} not found in input dataframe")


if __name__ == "__main__":
    main()