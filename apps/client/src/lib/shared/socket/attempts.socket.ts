import { browser } from "$app/environment"
import { io, type Socket } from "socket.io-client"
import { sessionManager } from "$lib/shared/auth/session.manager"
import type { AttemptSubmitView } from "$lib/attempts/types"

type AttemptsSocket = Socket

const SOCKET_PATH = "/api/socket.io"
const NAMESPACE = "/attempts"

let socket: AttemptsSocket | null = null

const getSocket = () => {
	if (!browser) return null

	if (!socket) {
		socket = io(NAMESPACE, {
			autoConnect: false,
			path: SOCKET_PATH,
			transports: ["polling", "websocket"],
		})
	}

	return socket
}

export const connectAttemptsSocket = () => {
	const currentSocket = getSocket()

	if (!currentSocket) return null

	const accessToken = sessionManager.getAccessToken()
	currentSocket.auth = accessToken
		? {
				authorization: `Bearer ${accessToken}`,
			}
		: {}

	if (!currentSocket.connected) {
		currentSocket.connect()
	}

	return currentSocket
}

export const disconnectAttemptsSocket = () => {
	if (!socket) return

	socket.disconnect()
}

export const onAttemptsSubmit = (handler: (payload: AttemptSubmitView) => void) => {
	const currentSocket = connectAttemptsSocket()

	if (!currentSocket) return () => {}

	currentSocket.on("attempts:new-submit", handler)

	return () => {
		currentSocket.off("attempts:new-submit", handler)
	}
}
