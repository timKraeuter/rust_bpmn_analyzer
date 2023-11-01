#!/usr/bin/env python

"""Generate the scalability plot for the article."""

import argparse
import json
from scipy import stats
import matplotlib.pyplot as plt

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("file", help="JSON file with benchmark results")

args = parser.parse_args()

with open(args.file) as f:
    results = json.load(f)["results"]

all_means = [b["mean"] for b in results]
blocks = list(range(1, 301))

plt.plot(blocks, all_means, label="average runtime data")

# Linear regression
slope, intercept, r, p, std_err = stats.linregress(blocks, all_means)


def myfunc(x):
    return slope * x + intercept


mymodel = list(map(myfunc, blocks))
plt.plot(blocks, mymodel, label="linear regression")

# plt.title("HOT scalability")
plt.xlabel("Blocks per BPMN model")
plt.ylabel("Average runtime in s")
plt.legend()
plt.show()
