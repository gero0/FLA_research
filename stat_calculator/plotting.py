from matplotlib import pyplot as plt
import pandas as pd
from helpers import all_stats
from pathlib import Path
import argparse
import os


def draw_corr_matrix(df):
    f = plt.figure(figsize=(30, 30))
    plt.imshow(df, cmap="PRGn")
    plt.xticks(range(df.select_dtypes(['number']).shape[1]),
               df.select_dtypes(['number']).columns,
               fontsize=14,
               rotation=90)
    plt.yticks(range(df.select_dtypes(['number']).shape[1]),
               df.select_dtypes(['number']).columns,
               fontsize=14)
    cb = plt.colorbar()
    cb.ax.tick_params(labelsize=14)
    plt.title('Correlation Matrix', fontsize=16)

    for i in range(len(df.index)):
        for j in range(len(df.columns)):
            text = plt.text(j,
                            i,
                            round(df.iloc[j, i], 2),
                            ha="center",
                            va="center",
                            color="black")

    f.tight_layout()


def main():
    parser = argparse.ArgumentParser(prog="lonstats",
                                     description="What the program does")
    parser.add_argument("filename", help="Name of file containing input data")
    parser.add_argument("corr_filename",
                        help="Name of file containing input correlation data")
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

    for stat in df.columns[3:]:
        try:
            df.plot(backend="matplotlib", x="node_count", y=stat)
            plt.savefig(os.path.join(output, stat))
        except:
            print(f"Stat {stat} not found in input dataframe")

    corr = pd.read_csv(args.corr_filename, sep=";", index_col=0)
    print(corr)
    draw_corr_matrix(corr)
    plt.savefig(os.path.join(output, "corr.png"))


if __name__ == "__main__":
    main()