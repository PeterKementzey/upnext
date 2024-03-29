import io
import os
from pathlib import Path

import ruamel.yaml
import typer
from ruamel.yaml import yaml_object

app = typer.Typer()
yaml = ruamel.yaml.YAML()
current_working_directory = Path(os.getcwd())


@yaml_object(yaml)
class Series:
    def __init__(self, path: str, episode: int):
        self.path = path
        self.episode = episode


test_series = [
    Series("/home/peter/Downloads/torrents/Ted.Lasso.S02.1080p.WEBRip.x265-RARBG[eztv.re]", 1),
    Series("/home/peter/Videos/Movies_no_backup/Series/Archer Season 3/", 1),
]

yaml_path: Path = Path(os.environ.get("HOME")) / ".upnext.yaml"


def read_yaml_string():
    """
    Read the yaml file from the home folder. If the file does not exist, create it and return an empty string.
    """
    if not os.path.exists(yaml_path):
        open(yaml_path, 'w').close()
        print(f"Created yaml file at {yaml_path}")
        return ""
    with open(yaml_path) as file:
        yaml_string = file.read()
        print(f"Read {yaml_path}:\n{yaml_string}\n")
        return yaml_string


def write_yaml_string(yaml_string: str):
    """
    Write the yaml string to the yaml file in the home folder.
    """
    with open(yaml_path, 'w') as file:
        file.write(yaml_string)
        print(f"Wrote to {yaml_path}:\n{yaml_string}\n")


@app.callback()
def init():
    print("loading yaml file...")
    yaml_string = read_yaml_string()


@app.command()
def main():
    print("saving yaml file...")
    string_io = io.StringIO()
    yaml.dump(test_series, string_io)
    yaml_string = string_io.getvalue()
    write_yaml_string(yaml_string)


if __name__ == "__main__":
    app()
