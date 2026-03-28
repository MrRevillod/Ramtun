#!/usr/bin/env bash

set -e

echo "Setting up WireGuard..."

chmod +x /app/apps/server/config/scripts/vpn.sh
chmod +x /app/apps/server/config/scripts/entrypoint.sh

if ! /app/apps/server/config/scripts/vpn.sh; then
    echo "Error setting up VPN, continuing without it..."
else
    echo "VPN configured successfully"
fi

echo "Starting..."
cd apps/server && cargo watch '-x run --bin server' -w src -w config
