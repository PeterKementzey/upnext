import os
from pathlib import Path
from sys import stdout

import ruamel.yaml
from ruamel.yaml import yaml_object

yaml = ruamel.yaml.YAML()
yaml_path = Path(os.environ.get("HOME")) / ".upnext.yaml"


@yaml_object(yaml)
class Series:
    def __init__(self, path: str, episode: int):
        self.path = path
        self.episode = episode


def read_yaml_string() -> None:
    """
    Read the yaml file from the home folder. If the file does not exist, create it and return an empty string.
    """
    global data
    if not os.path.exists(yaml_path):
        open(yaml_path, 'w').close()
        print(f"Created yaml file at {yaml_path}")
        return
    with open(yaml_path) as file:
        data = yaml.load(file)
        print(f"Read {yaml_path}:\n")
        yaml.dump(data, stdout)
        print("\n")
        return


def write_yaml_string() -> None:
    """
    Write the yaml string to the yaml file in the home folder.
    """
    global data
    with open(yaml_path, 'w') as file:
        yaml.dump(data, file)
        print(f"Wrote to {yaml_path}:\n")
        yaml.dump(data, stdout)
        print("\n")
