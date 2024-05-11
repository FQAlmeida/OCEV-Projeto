import numpy as np
import plotly.express as px


def nqueen_plotter(solution: list[int | float]):
    cols = np.array(list(map(int, solution)))
    board_size = len(solution)
    board = np.zeros((board_size, board_size))
    board[list(range(board_size)), cols] = 1

    for i, queen in enumerate(solution[:-1]):
        for j, next_queen in enumerate(solution[i + 1 :]):
            offset = j + 1
            if queen < offset or queen + offset >= board_size:
                continue
            if next_queen in {queen - offset, queen + offset}:
                board[i][int(queen)] = -1
                board[j][int(next_queen)] = -1
    my_color_scale = ["red", "white", "black"]
    return px.imshow(
        board,
        zmin=-1,
        zmax=1,
        color_continuous_scale=my_color_scale,
        labels=dict(x="Colunas", y="Linhas"),
    )


problems_dict = {
    "N-QUEENS": nqueen_plotter,
}
