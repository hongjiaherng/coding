# neetcode150-py

## Getting Started

```bash
uv venv
./venv/Scripts/activate
uv pip compile pyproject.toml -o requirements-dev.txt --extra=dev # uv pip compile pyproject.toml -o requirements.txt
uv pip sync requirements.txt
```

```bash
# Test specific file
pytest tests/neetcode150_py/arrays_and_hashing/test_q217_contains_duplicate.py

# Test specific file with print
pytest tests/neetcode150_py/arrays_and_hashing/test_q217_contains_duplicate.py -rP

# Test all
pytest
# or
make test

# Test all files, and generate coverage report
pytest --cov=src/neetcode150_py --cov-report=term-missing --cov-report=html
# or
make test-cov
```

```bash
# Everything in makefile
make check
make format
```

```bash
# Install own package with dependencies only
uv pip install -e .

# Install own package with dev dependencies
uv pip install -e .[dev]
```
