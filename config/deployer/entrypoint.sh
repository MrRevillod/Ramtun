#!/usr/bin/env sh
set -eu

connect_vpn() {
	if [ "${SKIP_VPN:-0}" = "1" ]; then
		return
	fi

	bash /opt/scripts/vpn.sh
}

resolve_kubeconfig() {
	if [ -n "${KUBECONFIG:-}" ] && [ -f "${KUBECONFIG}" ]; then
		return
	fi

	if [ -f "/workspace/config/deployer/kubectl/kubeconfig" ]; then
		export KUBECONFIG="/workspace/config/deployer/kubectl/kubeconfig"
		return
	fi

	if [ -f "/workspace/config/kubectl/kubeconfig" ]; then
		export KUBECONFIG="/workspace/config/kubectl/kubeconfig"
		return
	fi

	echo "[deploy] kubeconfig not found" >&2
	exit 1
}

run_deploy() {
	NAMESPACE="${K8S_NAMESPACE:-lrevillod}"
	ENV_FILE="${DEPLOY_ENV_FILE:-/workspace/.env}"
	MANIFESTS_DIR="${K8S_MANIFESTS_DIR:-/workspace/config/deployer/k8s}"

	echo "[deploy] namespace: ${NAMESPACE}"
	echo "[deploy] manifests: ${MANIFESTS_DIR}"

	kubectl get namespace "${NAMESPACE}" >/dev/null 2>&1 || kubectl create namespace "${NAMESPACE}"

	if [ -f "${ENV_FILE}" ]; then
		echo "[deploy] syncing secret ramtun-env from ${ENV_FILE}"
		kubectl -n "${NAMESPACE}" create secret generic ramtun-env \
			--from-env-file="${ENV_FILE}" \
			--dry-run=client -o yaml | kubectl apply -f -
	else
		echo "[deploy] warning: ${ENV_FILE} not found; secret sync skipped"
	fi

	kubectl -n "${NAMESPACE}" apply -R -f "${MANIFESTS_DIR}"

	kubectl -n "${NAMESPACE}" rollout status deployment/ramtun-server --timeout=180s
	kubectl -n "${NAMESPACE}" rollout status deployment/ramtun-client --timeout=180s

	echo "[deploy] current resources"
	kubectl -n "${NAMESPACE}" get deploy,svc,ingress,pods
}

connect_vpn
resolve_kubeconfig

if [ "$#" -eq 0 ] || [ "$1" = "deploy" ]; then
	run_deploy
	exit 0
fi

exec "$@"
