import os
import pprint
import re
from datetime import datetime
from pathlib import Path

import plotly.express as px
import polars as pl
import streamlit as st
import numpy as np
from problems_repr_plotter import problems_dict
from read_log_file import read_log_file

st.set_page_config(st.session_state.get("title", "Resultados"), "🧬", "wide")


files = list(filter(lambda x: x.is_file(), Path("../data/outputs").glob("*.log")))
most_recent_files = sorted(files, key=os.path.getmtime, reverse=True)


def format_func(x):
    formatter = re.compile("application_(.+)\\.log")
    match = formatter.search(x.name)
    if not match:
        return x.name
    return datetime.fromisoformat(match.group(1)).strftime("%d/%m/%Y %H:%M:%S")


current_file = st.selectbox(
    "Selecione o arquivo de log",
    most_recent_files,
    format_func=format_func,
    index=0,
)


@st.cache_data
def get_data(current_file: Path):
    problem_data = read_log_file(current_file)
    data = problem_data.runs
    results = [d.best for d in data]
    convergency = [
        pl.DataFrame({
            "generation": [i.generation for i in d.generations],
            "Best of All": [i.best_all for i in d.generations],
            "Population's Best": [i.best_pop for i in d.generations],
            "Population's Mean": [i.mean for i in d.generations],
            "Population's Worst": [i.worst for i in d.generations],
        })
        for d in data
    ]

    return problem_data, results, convergency


if not current_file:
    st.warning("Nenhum arquivo de log encontrado")
    st.stop()

problem_data, results, convergency = get_data(current_file)
if not st.session_state.get("title"):
    st.session_state.title = f"Resultados do {problem_data.name.upper()}"
    st.rerun()
st.title(f"Resultados do {problem_data.name.upper()}")

st.markdown("### Configuração")
st.json(problem_data.config, expanded=False)

st.plotly_chart(
    px.box(
        y=results,
        title="Resultados",
        labels={"y": "Resultados de cada Execução"},
    ),
    use_container_width=True,
)
fitness = np.array(list(map(lambda run: run.best_normed ,problem_data.runs)))
best_all_runs = problem_data.runs[np.argmax(fitness)]
st.dataframe(
    pl.DataFrame({
        "Mean": fitness.mean(),
        "Std": fitness.std(),
        "Best Value Human-Readable": best_all_runs.best,
        "Best Human-Readable": pprint.pformat(best_all_runs.decoded),
    }),
    use_container_width=True,
    hide_index=True,
)

for i, d in enumerate(convergency):
    st.markdown(f"### Estatísticas da Execução {i + 1}")
    st.dataframe(
        pl.DataFrame({
            "Best Human-Readable": pprint.pformat(problem_data.runs[i].decoded),
            "Best": pprint.pformat(problem_data.runs[i].best_individual),
            "Best Value Human-Readable": problem_data.runs[i].best,
            "Best Value": problem_data.runs[i].best_normed,
            "Has Constraint": problem_data.runs[i].constraint != 0,
            "Constraint": problem_data.runs[i].constraint,
        }),
        use_container_width=True,
        hide_index=True,
    )
    if problem_data.name.upper() in problems_dict:
        st.plotly_chart(
            problems_dict[problem_data.name.upper()](problem_data.runs[i].decoded),
            use_container_width=True,
        )
    st.plotly_chart(
        px.line(
            data_frame=d.to_pandas(),
            x="generation",
            y=[
                "Best of All",
                "Population's Best",
                "Population's Mean",
                "Population's Worst",
            ],
            labels={"value": "Fitness", "generation": "Generation"},
        ),
        use_container_width=True,
    )
