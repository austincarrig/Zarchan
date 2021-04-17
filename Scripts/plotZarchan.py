#! /usr/bin/env /home/acarri/anaconda3/bin/python

import os
import pandas as pd
import matplotlib.pyplot as plt

SCRIPT_DIR = os.path.abspath(os.path.dirname(__file__))
ZARCHAN_DIR = os.path.dirname(SCRIPT_DIR)

A = pd.read_csv(os.path.join(ZARCHAN_DIR, 'output', 'data.txt'), delim_whitespace=True)

plt.plot(A['rt1'], A['rt2'])
plt.plot(A['rm1'], A['rm2'])
plt.show()