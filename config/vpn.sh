#!/usr/bin/env bash

privatekey="$WIREGUARD_PRIVATE_KEY"
presharedkey="$WIREGUARD_PRESHARED_KEY"
wireguard_ip="$WIREGUARD_IP"
wireguard_peer="$WIREGUARD_PEER"
wireguard_endpoint="$WIREGUARD_ENDPOINT"
ldap_endpoint="$LDAP_ENDPOINT"
kubernetes_endpoint="$KUBERNETES_ENDPOINT"

tmp_privatekey="$(mktemp /tmp/wg-privatekey.XXXXXX)"
tmp_presharedkey="$(mktemp /tmp/wg-presharedkey.XXXXXX)"

cleanup() {
    rm -f "$tmp_privatekey" "$tmp_presharedkey"
}

trap cleanup EXIT

umask 077
printf '%s' "$privatekey" > "$tmp_privatekey"
printf '%s' "$presharedkey" > "$tmp_presharedkey"

ip link add dev wg0 type wireguard

ip address add dev wg0 "$wireguard_ip" peer "$wireguard_peer"

wg set wg0 private-key "$tmp_privatekey" \
 peer UBi3x7Cjv4lPABuWcbv7yTOgiDyb2ElLN+39J1gHqnU= preshared-key \
 "$tmp_presharedkey" endpoint "$wireguard_endpoint" \
 allowed-ips "$wireguard_peer"/32,"$ldap_endpoint","$kubernetes_endpoint"

ip link set up dev wg0

ip route add "$ldap_endpoint" via "$wireguard_peer" dev wg0
ip route add "$kubernetes_endpoint" via "$wireguard_peer" dev wg0
