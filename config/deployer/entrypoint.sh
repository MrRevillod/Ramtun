#!/usr/bin/env sh
set -eu

KUBECONFIG_PATH="/workspace/config/deployer/kubectl/kubeconfig"

connect_vpn() {
	if [ "${SKIP_VPN:-0}" = "1" ]; then
		return
	fi

	bash /opt/scripts/vpn.sh
}

normalize_env() {
	printf '%s' "$1" | sed -e 's/\r$//' -e ':a;s/^"\(.*\)"$/\1/;ta' -e ":b;s/^'\(.*\)'$/\1/;tb"
}

ensure_k8s_host_resolution() {
	server_url=$(awk '/^[[:space:]]*server:[[:space:]]/ {print $2; exit}' "$KUBECONFIG_PATH")
	if [ -z "${server_url:-}" ]; then
		return
	fi

	server_host=$(printf '%s' "$server_url" | sed -E 's#^https?://##' | cut -d/ -f1 | cut -d: -f1)
	if [ -z "${server_host:-}" ]; then
		return
	fi

	if grep -Eq "[[:space:]]${server_host}([[:space:]]|$)" /etc/hosts; then
		return
	fi

	endpoint=$(normalize_env "${KUBERNETES_ENDPOINT:-}")
	endpoint_ip=$(printf '%s' "$endpoint" | cut -d/ -f1)

	if [ -z "${endpoint_ip:-}" ]; then
		echo "[deploy] cannot resolve ${server_host} and KUBERNETES_ENDPOINT is empty" >&2
		exit 1
	fi

	echo "[deploy] adding host mapping ${server_host} -> ${endpoint_ip}"
	echo "${endpoint_ip} ${server_host}" >> /etc/hosts
}

run_deploy() {
	NAMESPACE="${K8S_NAMESPACE:-ramtun}"
	ENV_FILE="${DEPLOY_ENV_FILE:-/workspace/.env.prod}"
	MANIFESTS_DIR="${K8S_MANIFESTS_DIR:-/workspace/config/deployer/k8s}"
	POSTGRES_SECRET_NAME="${POSTGRES_SECRET_NAME:-postgres-secret}"
	POSTGRES_CLUSTER_NAME="${POSTGRES_CLUSTER_NAME:-postgres-cluster}"
	APP_SECRET_NAME="${APP_SECRET_NAME:-ramtun-secret}"

	echo "[deploy] namespace: ${NAMESPACE}"
	echo "[deploy] manifests: ${MANIFESTS_DIR}"

	kubectl get namespace "${NAMESPACE}" >/dev/null 2>&1 || kubectl create namespace "${NAMESPACE}"

	if [ -f "${ENV_FILE}" ]; then
		tmp_env_file=$(mktemp /tmp/ramtun-env.XXXXXX)
		trap 'rm -f "${tmp_env_file}"' EXIT
		postgres_user=""
		postgres_password=""
		jwt_secret=""

		while IFS= read -r line || [ -n "$line" ]; do
			case "$line" in
				''|\#*)
					continue
					;;
			esac

			key=$(printf '%s' "$line" | cut -d= -f1)
			if [ -z "$key" ] || [ "$key" = "$line" ]; then
				continue
			fi

			value=$(printf '%s' "$line" | cut -d= -f2-)
			value=$(normalize_env "$value")
			printf '%s=%s\n' "$key" "$value" >> "${tmp_env_file}"

			if [ "$key" = "POSTGRES_USER" ]; then
				postgres_user="$value"
			fi

			if [ "$key" = "POSTGRES_PASSWORD" ]; then
				postgres_password="$value"
			fi

			if [ "$key" = "JWT_SECRET" ]; then
				jwt_secret="$value"
			fi
		done < "${ENV_FILE}"

		echo "[deploy] syncing secret ramtun-env from ${ENV_FILE}"
		kubectl -n "${NAMESPACE}" create secret generic ramtun-env \
			--from-env-file="${tmp_env_file}" \
			--dry-run=client -o yaml | kubectl apply -f -

		if [ -n "$postgres_user" ] && [ -n "$postgres_password" ]; then
			echo "[deploy] syncing secret ${POSTGRES_SECRET_NAME} from ${ENV_FILE}"
			kubectl -n "${NAMESPACE}" create secret generic "${POSTGRES_SECRET_NAME}" \
				--from-literal=username="$postgres_user" \
				--from-literal=password="$postgres_password" \
				--dry-run=client -o yaml | kubectl apply -f -
		else
			echo "[deploy] warning: POSTGRES_USER or POSTGRES_PASSWORD missing in ${ENV_FILE}; ${POSTGRES_SECRET_NAME} was not updated"
		fi

		if [ -n "$jwt_secret" ]; then
			echo "[deploy] syncing secret ${APP_SECRET_NAME} from ${ENV_FILE}"
			kubectl -n "${NAMESPACE}" create secret generic "${APP_SECRET_NAME}" \
				--from-literal=secret="$jwt_secret" \
				--dry-run=client -o yaml | kubectl apply -f -
		else
			echo "[deploy] warning: JWT_SECRET missing in ${ENV_FILE}; ${APP_SECRET_NAME} was not updated"
		fi

		rm -f "${tmp_env_file}"
		trap - EXIT
	else
		echo "[deploy] warning: ${ENV_FILE} not found; secret sync skipped"
	fi

	if [ -d "${MANIFESTS_DIR}/postgres" ]; then
		echo "[deploy] applying postgres manifests"
		if ! kubectl -n "${NAMESPACE}" apply -R -f "${MANIFESTS_DIR}/postgres"; then
			echo "[deploy] failed applying postgres manifests (check CNPG RBAC/CRD permissions)" >&2
			exit 1
		fi

		kubectl -n "${NAMESPACE}" wait --for=condition=Ready "cluster.postgresql.cnpg.io/${POSTGRES_CLUSTER_NAME}" --timeout=300s || true
	fi

	echo "[deploy] applying application manifests"
	kubectl -n "${NAMESPACE}" apply -R -f "${MANIFESTS_DIR}/server"
	kubectl -n "${NAMESPACE}" apply -R -f "${MANIFESTS_DIR}/client"

	if ! kubectl -n "${NAMESPACE}" rollout status deployment/ramtun-server --timeout=180s; then
		echo "[deploy] rollout failed for ramtun-server" >&2
		kubectl -n "${NAMESPACE}" get pods -l app=ramtun-server -o wide || true
		kubectl -n "${NAMESPACE}" logs deployment/ramtun-server --tail=120 || true
		exit 1
	fi

	if ! kubectl -n "${NAMESPACE}" rollout status deployment/ramtun-client --timeout=180s; then
		echo "[deploy] rollout failed for ramtun-client" >&2
		kubectl -n "${NAMESPACE}" get pods -l app=ramtun-client -o wide || true
		kubectl -n "${NAMESPACE}" logs deployment/ramtun-client --tail=120 || true
		exit 1
	fi

	echo "[deploy] current resources"
	kubectl -n "${NAMESPACE}" get deploy,svc,ingress,pods
}

connect_vpn
if [ ! -f "$KUBECONFIG_PATH" ]; then
	echo "[deploy] kubeconfig not found at $KUBECONFIG_PATH" >&2
	exit 1
fi

export KUBECONFIG="$KUBECONFIG_PATH"
ensure_k8s_host_resolution

if [ "$#" -eq 0 ] || [ "$1" = "deploy" ]; then
	run_deploy
	exit 0
fi

exec "$@"
