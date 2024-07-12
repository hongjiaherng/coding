.PHONY: lint
lint:
	mypy src tests
	ruff check .

.PHONY: format
format:
	ruff check --fix .

.PHONY: test
test:
	pytest tests

.PHONY: test-cov
test-cov:
	pytest --cov=src --cov-report=term-missing --cov-report=html

.PHONY: check
check: lint test