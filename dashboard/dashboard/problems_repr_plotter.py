import numpy as np
import plotly.express as px
import streamlit as st


def nqueen_plotter(solution: list[int | float]):
    cols = np.array(list(map(int, solution)))
    board_size = len(solution)
    board = np.zeros((board_size, board_size))
    board[list(range(board_size)), cols] = 1
    return (
        px.imshow(
            board,
            zmin=0,
            zmax=1,
            color_continuous_scale=px.colors.sequential.gray_r,
            labels=dict(x="Colunas", y="Linhas"),
        )
        # .update_xaxes(
        #     showgrid=True,
        #     zeroline=True,
        #     gridwidth=20,
        #     gridcolor="Black",
        # )
        # .update_yaxes(
        #     showgrid=True,
        #     zeroline=True,
        #     gridwidth=20,
        #     gridcolor="Black",
        # )
    )


problems_dict = {
    "N-QUEENS": nqueen_plotter,
}
