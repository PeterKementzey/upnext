import logging as log
from sys import stdout
from typing import Optional

import typer
from typer import Typer
from typing_extensions import Annotated

from app_logic import AppLogic

log.basicConfig(format='%(levelname)s:%(message)s', stream=stdout, level=log.WARN)

app_name = "upnext"
app: Typer = typer.Typer(no_args_is_help=True, name=app_name, add_completion=False)

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
def remove():
    """
    Remove data about the series in current directory.
    :return:
    """
    app_logic.remove_current_series()
    print("Series removed.")


@app.command()
def edit():
    """
    Open the yaml file in the default editor.
    :return:
    """
    app_logic.edit_yaml_file()


@app.command("list")
def print_all():
    """
    Print all series information.
    :return:
    """
    app_logic.print_all()


@app.command()
def play(wait_in_sec: Annotated[int,
typer.Option("--wait", help="Number of seconds to wait before playing next episode.")] = 8):
    """
    Start playing series episodes.
    :return:
    """
    app_logic.print_info()
    if app_logic.is_over() is False:
        app_logic.play_next_episode()
        app_logic.increment_next_episode()
        app_logic.print_info()
    while app_logic.is_over() is False:
        app_logic.countdown_to_episode(wait_in_sec)
        app_logic.play_next_episode()
        app_logic.increment_next_episode()
        app_logic.print_info()
    series_over_actions()


@app.command("next")
def play_next_episode():
    """
    Play the next episode.
    :return:
    """
    app_logic.print_info()
    if app_logic.is_over() is False:
        app_logic.play_next_episode()
        app_logic.increment_next_episode()
        app_logic.print_info()
    else:
        series_over_actions()


if __name__ == "__main__":
    app()


def series_over_actions():
    print("Series over.")
    user_wants_to_reset = typer.confirm("Do you want to reset the series?")
    if user_wants_to_reset:
        remove()
