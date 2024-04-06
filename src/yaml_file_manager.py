import logging as log
import os
from pathlib import Path

import ruamel.yaml


class YamlFileManager:
    yaml_engine = ruamel.yaml.YAML()
    yaml_path: Path
    yaml_data: list[dict]

    def __init__(self, path: Path = Path.home() / ".upnext.yaml"):
        self.yaml_path = path
        self._load()

    def _log_yaml_data(self):
        if log.root.level == log.DEBUG:
            if not isinstance(log.root.handlers[0], log.StreamHandler):
                raise ValueError("log.root.handlers[0] is not a StreamHandler")
            log_handler: log.StreamHandler = log.root.handlers[0]
            self.yaml_engine.dump(self.yaml_data, log_handler.stream)

    def _load(self) -> None:
        """
        Read the yaml file from the home folder. If the file does not exist, create it and return an empty string.
        """
        if not os.path.exists(self.yaml_path):
            self.yaml_data = []
            open(self.yaml_path, 'w').close()
            log.info(f"Created yaml file at {self.yaml_path}")
            return
        with open(self.yaml_path) as file:
            self.yaml_data = self.yaml_engine.load(file)

            # validate(self.yaml_data)

            log.info(f"Read {self.yaml_path}:")
            self._log_yaml_data()
            return

    def save(self) -> None:
        """
        Write the yaml string to the yaml file in the home folder.
        """
        with open(self.yaml_path, 'w') as file:
            self.yaml_engine.dump(self.yaml_data, file)
            log.info(f"Wrote to {self.yaml_path}:")
            self._log_yaml_data()

    @staticmethod
    def _new_series(path: str) -> dict:
        """
        Create a new series with the given path.
        """
        return {
            "path": path,
            "next_episode": 1
        }

    def find_series_by_path(self, path: str) -> dict | None:
        """
        Find the series with the given path.
        """
        for series in self.yaml_data:
            if series["path"] == path:
                return series
        return None

    def create_series_by_path(self, path: str) -> dict:
        """
        Get the current series from the yaml file. Returns a pointer.
        """
        log.info("get_series_by_path:", path)
        series = self.find_series_by_path(path)
        if series:
            return series
        else:
            self.yaml_data.append(YamlFileManager._new_series(path))
            return self.yaml_data[-1]

    def remove_series_by_path(self, path: str) -> None:
        """
        Remove the series with the given path.
        """
        self.yaml_data = [s for s in self.yaml_data if s["path"] != path]
