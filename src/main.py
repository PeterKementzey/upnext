import logging as log
from sys import stdout
from typing import Optional

import typer
from typer import Typer
from typing_extensions import Annotated

from app_logic import AppLogic

log.basicConfig(format='%(levelname)s:%(message)s', stream=stdout, level=log.WARN)

app_name = "upnext"
app: Typer = typer.Typer()

app_logic = AppLogic()


@app.command()
def info():
    """
    Print the series information in the current directory.
    :return:
    """
    app_logic.print_info()


@app.command()
def init():
    """
    Initialize the current directory as a series.
    :return:
    """
    app_logic.initialize_directory()
    app_logic.print_info()


@app.command("set")
def set_next_episode(n: int):
    """
    Set the next episode number explicitly.
    :param n:
    :return:
    """
    app_logic.print_info()
    app_logic.set_next_episode(n)
    app_logic.print_info()


@app.command("inc")
def increment_episode_count(n: Annotated[Optional[int], typer.Argument()] = 1):
    """
    Increment the next episode number by n. Default is 1.
    :param n:
    :return:
    """
    app_logic.print_info()
    app_logic.increment_next_episode(n)
    app_logic.print_info()


@app.command()
def reset():
    """
    Remove data about the series in current directory.
    :return:
    """
    app_logic.remove_current_series()
    print("Series removed.")


@app.command()
def play(n: Annotated[Optional[int | None], typer.Argument()] = None):
    """
    Play n episodes starting from the next episode. If n is not provided, play until stopped.
    :return:
    """
    app_logic.print_info()
    if app_logic.is_over() is False and (n is None or n > 0):
        app_logic.play_next_episode()
        app_logic.increment_next_episode()
        app_logic.print_info()
        if n is not None:
            n -= 1
    while app_logic.is_over() is False and (n is None or n > 0):
        app_logic.countdown_to_episode(10)
        app_logic.play_next_episode()
        app_logic.increment_next_episode()
        app_logic.print_info()
        if n is not None:
            n -= 1


if __name__ == "__main__":
    app()
