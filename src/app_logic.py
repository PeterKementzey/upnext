import glob
import os
import subprocess
from pathlib import Path
from sys import stdout
from time import sleep

import ruamel.yaml

from yaml_file_manager import YamlFileManager


class AppLogic:
    app_name = "upnext"
    current_working_directory: Path = Path(os.getcwd())
    yaml_file_manager: YamlFileManager
    series: dict | None
    file_extensions = ["mkv", "mp4", "avi", "flv", "mov", "wmv", "webm", "mpg", "mpeg", "m4v"]

    def __init__(self):
        self.yaml_file_manager = YamlFileManager()
        self.series = self.yaml_file_manager.find_series_by_path(str(self.current_working_directory))

    def _ensure_series_not_null(self) -> dict:
        if self.series is None:
            raise ValueError(f"No series found. Please run '{self.app_name} init' first.")
        return self.series

    def print_info(self):
        series = self._ensure_series_not_null()
        print("---")
        ruamel.yaml.YAML().dump([series], stdout)

    def initialize_directory(self):
        if self.series:
            print("Current directory is already initialized.")
        else:
            print("Initializing current directory.")
            self.series = self.yaml_file_manager.create_series_by_path(str(self.current_working_directory))
            self.yaml_file_manager.save()

    def set_next_episode(self, n: int):
        series: dict = self._ensure_series_not_null()
        series["next_episode"] = n
        self.yaml_file_manager.save()

    def increment_next_episode(self, n: int = 1):
        series: dict = self._ensure_series_not_null()
        series["next_episode"] += n
        self.yaml_file_manager.save()

    def remove_current_series(self):
        self.yaml_file_manager.remove_series_by_path(str(self.current_working_directory))
        self.series = None
        self.yaml_file_manager.save()

    def is_over(self):
        series: dict = self._ensure_series_not_null()
        files = self._find_files(series["path"], self.file_extensions)
        return series["next_episode"] > len(files)

    def play_next_episode(self):
        series: dict = self._ensure_series_not_null()
        files = self._find_files(series["path"], self.file_extensions)
        if self.is_over():
            raise ValueError("No more episodes to watch.")
        else:
            file_path = files[series["next_episode"] - 1]
            self._play_file_in_vlc(file_path)

    @staticmethod
    def _find_files(directory, extensions):
        files = []
        for ext in extensions:
            pattern = os.path.join(directory, f"*.{ext}")
            pattern = pattern.replace('[', '[[]')  # escape special characters
            files.extend(glob.glob(pattern))
        return sorted(files)

    @staticmethod
    def _play_file_in_vlc(file_path):
        log_file = Path.home() / ".upnext-vlc.log"
        with open(log_file, "a") as f:
            f.write(f"\n\nPlaying {file_path}\n")
            subprocess.call(["vlc", file_path, "--play-and-exit", "--fullscreen"], stdout=f, stderr=f)

    @staticmethod
    def countdown_to_episode(n: int):
        print("Playing next episode in...")
        for i in range(n, 0, -1):
            print(i)
            sleep(1)
        print("0")
