import os
from pathlib import Path
from sys import stdout

import ruamel.yaml
from ruamel.yaml import yaml_object
from schema import Schema, And, Use

yaml = ruamel.yaml.YAML()
yaml_path = Path(os.environ.get("HOME")) / ".upnext.yaml"

series_schema = Schema({
    "path": And(str, lambda p: len(p) > 0, error="path should be a non-empty string"),
    "next_episode": And(Use(int), lambda n: n > 0, error="next_episode should be a positive integer")
})


@yaml_object(yaml)
class Series:
    def __init__(self, path: str, next_episode: int):
        self.path = path
        self.next_episode = next_episode

    def to_dict(self):
        return vars(self)


def read_yaml_string() -> None:
    """
    Read the yaml file from the home folder. If the file does not exist, create it and return an empty string.
    """
    if not os.path.exists(yaml_path):
        open(yaml_path, 'w').close()
        print(f"Created yaml file at {yaml_path}")
        return
    with open(yaml_path) as file:
        global data
        data = yaml.load(file)

        data_schema = Schema([series_schema])
        data_prepared_for_validation: list[dict] = [series.to_dict() for series in data]
        _ = data_schema.validate(data_prepared_for_validation)

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


def get_series_by_path(path: str) -> Series | None:
    """
    Get the current series from the yaml file.
    """
    global data
    series: Series
    for series in data:
        if series.path == path:
            return series
    return None
