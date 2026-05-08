ifneq (,$(wildcard ./.env))
    include .env
    export
endif

SERVER=apps/server
CLIENT=apps/client
app ?= "server"
pkg ?=
service ?= $(app)
file ?=
cmd ?= deploy
args ?=
docker_tty ?= -it

.DEFAULT_GOAL := run
.PHONY: run detach clean fmt lint migration machete clean-db db npmi npmu npmci setup deployer

setup:
	rm -rf $(CLIENT)/node_modules
	cd $(CLIENT) && corepack pnpm install --frozen-lockfile --ignore-scripts
	docker compose build
	docker compose up client -d --force-recreate --renew-anon-volumes

run:
	docker compose up

detach:
	docker compose up -d

clean:
	docker compose down -v

clean-db:
	docker exec ramtun-postgres psql -U user -d database -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
	cd $(SERVER) && sqlx migrate run --source ./config/migrations --database-url $(LOCAL_POSTGRES_DATABASE_URL)

db:
	docker exec -it ramtun-postgres /bin/bash -c "psql -U user -d ramtun"

fmt:
	cargo fmt --all
	cd $(CLIENT) && corepack pnpm run format

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cd $(CLIENT) && corepack pnpm run lint

migration:
	cd $(SERVER) && sqlx migrate add --source ./config/migrations "$(name)"

machete:
	cargo machete

npmi:
	cd apps/$(app) && corepack pnpm add $(pkg)
	docker compose exec $(service) /bin/sh -c "cd /app/apps/$(app) && corepack pnpm add $(pkg)"

npmu:
	cd $(CLIENT) && corepack pnpm remove $(pkg)
	docker compose exec $(service) /bin/sh -c "cd /app/apps/$(app) && corepack pnpm remove $(pkg)"

npmci:
	cd $(CLIENT) && corepack pnpm install --frozen-lockfile --ignore-scripts
	docker compose exec $(service) /bin/sh -c "cd /app/apps/$(app) && corepack pnpm install --frozen-lockfile --ignore-scripts"

deployer:
	docker build -f config/deployer/Dockerfile -t questions-deployer:local .
	docker run --rm $(docker_tty) --cap-add=NET_ADMIN --device=/dev/net/tun --env-file .env.prod -v "$(PWD)":/workspace -v "$$HOME/.kube":/root/.kube:ro questions-deployer:local $(cmd) $(args)
