# pysimplicityhl
a python wrapper for the simplicityhl compiler

# Installation

First build the complete rust project
```bash
cargo build
```
and you should find the binaries in the subfolder `target` with target folder `debug` or `release`.

Now create a python environment ...
```bash
python -m venv .venv
```
... and activate it
```bash
. .venv/bin/activate # linux/mac 
. .venv/Scripts/activate # windows (git-bash)
```

Let poetry add missing libraries with
```bash
poetry update
```

Now build the debug wheel (to be found in folder debug/wheel)
```bash
maturin build --manifest-path pysimplicityhl/Cargo.toml
```
or the release build  (to be found in folder relöease/wheel)
```bash
maturin build --manifest-path pysimplicityhl/Cargo.toml --release
```

A quick install (i.e. a forced reinstall) of the newly compiled wheel could look like this, assuming at least one whl file exists.
```bash
poetry run pip install --force-reinstall "$(ls -t target/wheels/pysimplicityhl*.whl | head -n 1)"
```

If you want to do all at once you may test this `poe` command (make sure the `venv` is activated)
```bash
poetry run poe build-wheel
```

# Test

the demo file `demo/simple.hl` looks like this:
```rust
fn main() {}
```

this example `demo/main.py` will compile it when running 
```bash
poetry run python demo/main.py
```
and should give
```json
{"status":"success","program":"JA=="}
```
