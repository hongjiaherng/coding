# neetcode150-py

## Getting Started

```bash
uv venv
./venv/Scripts/activate
uv pip compile pyproject.toml -o requirements.txt
uv pip sync requirements.txt
```

```bash
# Test specific file
pytest tests/neetcode150_py/arrays_and_hashing/test_q217_contains_duplicate.py

# Test all
pytest

# Test all files, and generate coverage report
pytest --cov=src/neetcode150_py --cov-report=term-missing --cov-report=html
```

```bash
# Everything in makefile
make check
make format
```
