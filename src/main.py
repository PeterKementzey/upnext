import os
from pathlib import Path

import typer

from yaml_service import read_yaml_string, write_yaml_string

app = typer.Typer()
current_working_directory = Path(os.getcwd())


@app.callback()
def init():
    print("loading yaml file...")
    read_yaml_string()


@app.command()
def main():
    print("saving yaml file...")
    write_yaml_string()


if __name__ == "__main__":
    app()
