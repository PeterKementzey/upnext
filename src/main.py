import os
from pathlib import Path

import typer

import yaml_service

app = typer.Typer()
current_working_directory = Path(os.getcwd())


@app.callback()
def init():
    print("loading yaml file...")
    yaml_service.read_yaml_string()


@app.command()
def save():
    print("saving yaml file...")
    yaml_service.write_yaml_string()


@app.command()
def info():
    series = yaml_service.get_series_by_path(str(current_working_directory))
    if series is None:
        print("No series found.")
        return

    print(f"Next episode: {series.next_episode}")


if __name__ == "__main__":
    app()
