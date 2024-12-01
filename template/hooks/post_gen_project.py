import os
import sys

generated_root = os.getcwd()  # this is the folder of "cookiecutter.project_slug"

cargo_toml = os.path.join(generated_root, '..', '..', 'Cargo.toml')

try:
    with open(cargo_toml, 'a+') as file:
        file.seek(0, 2)  # Move to the end of the file
        if file.tell() == 0:
            raise FileNotFoundError
        # Append to the file here
        file.writelines([
            '\n',
            '[[bin]]\n',
            'name = "day_{{ cookiecutter.day }}"\n',
            'path = "src/day_{{ cookiecutter.day }}/main.rs"\n'
        ])

except FileNotFoundError:
    print(f"Could not find Cargo.toml at {cargo_toml}")
    sys.exit(1)