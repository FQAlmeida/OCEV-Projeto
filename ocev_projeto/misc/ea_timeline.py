import matplotlib.pyplot as plt
import numpy as np
import polars as pl
from matplotlib.axes import Axes

if __name__ == "__main__":
    df_ea = pl.read_excel("data/EA.xlsx")
    ax: Axes
    fig, ax = plt.subplots(figsize=(75, 5), constrained_layout=True)
    _ = ax.set_ylim(-0.5, 5)
    x_min: float = df_ea["Ano"].to_numpy().astype(np.float32).min()
    x_max: float = df_ea["Ano"].to_numpy().astype(np.float32).max()
    _ = ax.set_xlim(left=x_min, right=x_max)
    _ = ax.axhline(0, xmin=0.05, xmax=0.95, c='deeppink', zorder=1)

    df_agged = df_ea.group_by("Ano").agg("Nome")
    dates = df_agged["Ano"].to_numpy()
    nomes = df_agged["Nome"].to_numpy()
    labels = ["\n".join(n) for n in nomes]
    _ = ax.scatter(dates, np.zeros(len(dates)), s=120, c="palevioletred", zorder=2)
    _ = ax.scatter(dates, np.zeros(len(dates)), s=30, c="darkmagenta", zorder=3)
    label_offsets = np.zeros(len(dates))
    label_offsets[::2] = 0.35
    label_offsets[1::2] = -0.7

    labels = [f"{d}\n{label}" for label, d in zip(labels, dates)]
    for i, (label, d) in enumerate(zip(labels, dates)):
        _ = ax.text(
            d,
            label_offsets[i],
            label,
            ha="center",
            fontfamily="serif",
            fontweight="bold",
            color="royalblue",
            fontsize=12,
        )
    stems = np.zeros(len(dates))
    stems[::2] = 0.3
    stems[1::2] = -0.3
    markerline, stemline, baseline = ax.stem(dates, stems)
    _ = plt.setp(markerline, marker=",", color="darkmagenta")
    _ = plt.setp(stemline, color="darkmagenta")
    for spine in ["left", "top", "right", "bottom"]:
        _ = ax.spines[spine].set_visible(False)

    # hide tick labels
    _ = ax.set_xticks([])
    _ = ax.set_yticks([])

    _ = ax.set_title(
        "Important Milestones in Rock and Roll", fontweight="bold", fontfamily="serif", fontsize=16, color="royalblue"
    )
    fig.tight_layout()
    fig.savefig("data/mat.png")
