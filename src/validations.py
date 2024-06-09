from schema import Schema, And, Use, SchemaError

series_schema = Schema({
    "path": And(str, lambda p: len(p) > 0, error="path should be a non-empty string"),
    "next_episode": And(Use(int), lambda n: n > 0, error="next_episode should be a positive integer")
})

data_schema = Schema([series_schema])


def validate(yaml_data: list[dict]) -> None:
    try:
        data_schema.validate(yaml_data)
    except SchemaError as e:
        print("Error in yaml file:")
        for message in e.autos:
            if message is not None:
                print(f"  {message}")
        exit(e.code)
