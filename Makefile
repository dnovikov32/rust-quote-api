SHELL := /bin/bash
DEFAULT_GOAL := help
THIS_FILE := $(lastword $(MAKEFILE_LIST))

help:
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-27s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: start
start: ## Start application
	@echo -e "\033[33mstart\033[0m"
	@USERID=$$(id -u) GROUPID=$$(id -g) docker-compose --file docker-compose.yml up

.PHONY: build
build: ## Build application
	@echo -e "\033[33mstart\033[0m"
	@USERID=$$(id -u) GROUPID=$$(id -g) docker-compose --file docker-compose.yml build

.PHONY: rebuild
rebuild: ## Build application from scratch, without cache etc.
	@USERID=$$(id -u) GROUPID=$$(id -g) docker-compose --file docker-compose.yml rm -sf && \
	@USERID=$$(id -u) GROUPID=$$(id -g) docker-compose --file docker-compose.yml build --pull --no-cache && \
	@USERID=$$(id -u) GROUPID=$$(id -g) docker-compose --file docker-compose.yml up --force-recreate

.PHONY: clean
clean: ## Clean containers and application cache
	@echo -e "\033[33mclean\033[0m"
	@echo -e "\033[33mclean-docker\033[0m"
	@USERID=$$(id -u) GROUPID=$$(id -g) docker-compose --file docker-compose.yml down --volumes --remove-orphans --rmi local
	@if [ "$$(docker images -q -f dangling=true)" != "" ]; then docker rmi $$(docker images -q -f dangling=true) ;fi
