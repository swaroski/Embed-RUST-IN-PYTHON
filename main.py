# main.py
import pandas as pd
import seaborn as sns
from matplotlib import pyplot as plt
from mult import mult_matrix_func as mult_matrix_rust

from mult import mult_matrix_func as mult_matrix_python

if __name__ == "__main__":
    N = 800
    index = [2, 4, 8]
    index.extend([i for i in range(10, N, 5)])
    list_times_rust = []
    list_times_python = []
    for i in index:
        list_times_python.append(mult_matrix_python(i))
        list_times_rust.append(mult_matrix_rust(i))

    df = pd.DataFrame({"python": list_times_python, "rust": list_times_rust})
    df["difference"] = df.python - df.rust

    sns.lineplot(df)
    plt.show()
    plt.savefig("plot1.png")  # Save the plot as an image file