FROM alpine:3.21

RUN apk add --no-cache \
	bash \
	ca-certificates \
	curl \
	iproute2 \
	kubectl \
	wireguard-tools

WORKDIR /workspace

COPY config/vpn.sh /opt/scripts/vpn.sh
COPY config/deployer/entrypoint.sh /opt/scripts/entrypoint.sh

RUN chmod +x /opt/scripts/vpn.sh /opt/scripts/entrypoint.sh

ENTRYPOINT ["/opt/scripts/entrypoint.sh"]
