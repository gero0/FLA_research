import argparse
import subprocess
import pandas as pd
import natsort
import os
from os.path import isfile, join
from helpers import all_stats

MAX_THREADS = 4


def load(path):
    files = [f for f in os.listdir(path) if isfile(join(path, f))]
    files = natsort.natsorted(files)
    return files


def main():
    parser = argparse.ArgumentParser(prog="lonstats",
                                     description="What the program does")
    parser.add_argument("dirname",
                        help="Name of directory containing input files")
    parser.add_argument(
        "-s",
        "--stats",
        nargs="+",
        help="Stats to calculate. Leave empty to calculate all",
        required=False)
    parser.add_argument("-o",
                        "--output",
                        help="Name of output file",
                        required=False)

    parser.add_argument("-l",
                        "--limit",
                        help="limit input files to process",
                        required=False)

    args = parser.parse_args()

    path = args.dirname
    stats = args.stats
    output = args.output

    limit = args.limit
    if limit is None:
        limit = 999999999999999
    else:
        limit = int(args.limit)

    if output is None:
        output = "results.csv"

    if stats is None:
        stats = all_stats

    df = pd.DataFrame()
    files = load(path)

    n_files = min(len(files), limit)

    proc = []
    active_proc = []

    for i in range(n_files):
        filepath = join(path, files[i])
        command = [
            "python",
            os.path.realpath(os.path.dirname(__file__)) + "/calc_file.py",
            str(i), filepath
        ]
        for stat in stats:
            command.append(stat)
        print(
            f"=Starting thread {i}, working on file: {filepath}==================="
        )
        p = subprocess.Popen(command, stdout=subprocess.PIPE)
        active_proc.append((i, p))
        proc.append(p)

        #Wait until a thread is free
        while (len(active_proc) >= MAX_THREADS):
            #Remove terminated processes from active proc list to allow for new processes to start
            for (i, x) in active_proc:
                x.poll()
                if x.returncode is not None:
                    print(f"=Thread {i} finished! ===================")

            active_proc = [(i, x) for (i, x) in active_proc
                           if x.returncode is None]

    print(
        f"No files left to schedule. Waiting for remaining {len(active_proc)} processes to finish..."
    )
    for p in proc:
        p.wait()

    print(f"=All remaining threads finished! ===================")

    for index, p in enumerate(proc):
        (stdout, stderr) = p.communicate()
        s = stdout.decode("utf-8").strip()
        s = s.replace("'", "\"")
        s = s.replace("nan", "0")
        print(s)
        row = pd.read_json(s, typ="series")

        df = pd.concat([df, pd.DataFrame([row], index=[index])])

    print(df)

    df.to_csv(output, sep=";")
    corr = df.corr()
    corr.to_csv(output.replace(".csv", "_corr.csv"), sep=";")


if __name__ == "__main__":
    main()