import unittest
from pathlib import Path

from src.yaml_file_manager import YamlFileManager


class TestYamlFileManager(unittest.TestCase):
    example_file_path = Path("resources/example.yaml")
    example_data = [
        {"path": "/home/user/Videos/Series/Archer Season 3/", "next_episode": 1},
        {"path": "/home/user/Downloads/Avatar the Last Airbender Book 1/", "next_episode": 8}
    ]

    def reset_file(self):
        self.test_write_yaml_string_existing_file()

    def test_read_yaml_string_existing_file(self):
        yaml_file_manager = YamlFileManager(self.example_file_path)
        self.assertEqual(len(yaml_file_manager.yaml_data), 2)
        self.assertDictEqual(yaml_file_manager.yaml_data[0], self.example_data[0])
        self.assertDictEqual(yaml_file_manager.yaml_data[1], self.example_data[1])

    # def test_read_yaml_string_non_existing_file(self):

    def test_write_yaml_string_existing_file(self):
        with open(self.example_file_path, 'w') as file:
            file.write('')
        yaml_file_manager = YamlFileManager(self.example_file_path)
        yaml_file_manager.yaml_data = self.example_data
        yaml_file_manager.save()
        self.test_read_yaml_string_existing_file()

    def test__new_series(self):
        yaml_file_manager = YamlFileManager()
        series = yaml_file_manager._new_series("/test/path")
        self.assertEqual(series['path'], '/test/path')
        self.assertEqual(series['next_episode'], 1)

    # def test_find_series_by_path_existing_series(self):
    #     yaml_file_manager = YamlFileManager(self.example_file_path)
    #     yaml_file_manager.yaml_data.append({"path": "/test/path", "next_episode": 1})
    #     series = yaml_file_manager.find_series_by_path("/test/path")
    #     self.assertIsNotNone(series)
    #     self.assertEqual(series['path'], '/test/path')
    #
    # def test_find_series_by_path_non_existing_series(self):
    #     yaml_file_manager = YamlFileManager()
    #     series = yaml_file_manager.find_series_by_path("/test/path")
    #     self.assertIsNone(series)
    #
    # def test_create_series_by_path_existing_series(self):
    #     yaml_file_manager = YamlFileManager()
    #     yaml_file_manager.yaml_data.append({"path": "/test/path", "next_episode": 1})
    #     series = yaml_file_manager.create_series_by_path("/test/path")
    #     self.assertIsNotNone(series)
    #     self.assertEqual(series['path'], '/test/path')
    #
    # def test_create_series_by_path_non_existing_series(self):
    #     yaml_file_manager = YamlFileManager()
    #     series = yaml_file_manager.create_series_by_path("/test/path")
    #     self.assertIsNotNone(series)
    #     self.assertEqual(series['path'], '/test/path')
    #     self.assertEqual(len(yaml_file_manager.yaml_data), 1)


if __name__ == '__main__':
    unittest.main()
