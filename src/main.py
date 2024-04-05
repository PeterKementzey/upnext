import logging as log
import os
from pathlib import Path
from sys import stdout
from typing import Optional

import ruamel.yaml
import typer
from typer import Typer
from typing_extensions import Annotated

from yaml_file_manager import YamlFileManager

log.basicConfig(format='%(levelname)s:%(message)s', stream=stdout, level=log.DEBUG)

app_name = "upnext"
app: Typer = typer.Typer()

current_working_directory: Path = Path(os.getcwd())
yaml_file_manager: YamlFileManager = YamlFileManager()
series: dict | None = yaml_file_manager.find_series_by_path(str(current_working_directory))


def _ensure_series_not_null() -> dict:
    if series is None:
        raise ValueError(f"No series found. Please run '{app_name} init' first.")
    return series


@app.command()
def info():
    """
    Print the series information in the current directory.
    :return:
    """
    log.debug("COMMAND: info")
    series: dict = _ensure_series_not_null()
    print("---")
    ruamel.yaml.YAML().dump([series], stdout)


@app.command()
def init():
    """
    Initialize the current directory as a series.
    :return:
    """
    log.debug("COMMAND: init")
    global series
    if series:
        print("Current directory is already initialized.")
        info()
    else:
        print("Initializing current directory.")
        series = yaml_file_manager.create_series_by_path(str(current_working_directory))
        yaml_file_manager.save()
        info()


@app.command("set")
def set_next_episode(n: int):
    """
    Set the next episode number explicitly.
    :param n:
    :return:
    """
    log.debug(f"COMMAND: set {n}")
    series: dict = _ensure_series_not_null()
    # TODO: validate n - not too big?
    info()
    series["next_episode"] = n
    yaml_file_manager.save()
    info()


@app.command("watch")
def increment_episode_count(n: Annotated[Optional[int], typer.Argument()] = 1):
    """
    Increment the next episode number by n. Default is 1.
    :param n:
    :return:
    """
    log.debug(f"COMMAND: watch {n}")
    series = _ensure_series_not_null()
    set_next_episode(series["next_episode"] + n)


@app.command()
def reset():
    """
    Remove data about the series in current directory.
    :return:
    """
    log.debug("COMMAND: reset")
    _ensure_series_not_null()
    yaml_file_manager.remove_series_by_path(str(current_working_directory))
    yaml_file_manager.save()
    global series
    series = None


@app.command()
def play():
    log.debug("COMMAND: play")
    series = _ensure_series_not_null()
    info()
    raise NotImplementedError("Not yet implemented!")


if __name__ == "__main__":
    app()
