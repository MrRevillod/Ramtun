import { io, type Socket } from "socket.io-client"
import type { AttemptSubmitView, AttemptWarning } from "$lib/attempts/dtos"

type AttemptsSocket = Socket

const SOCKET_PATH = "/api/socket.io"
const NAMESPACE = "/attempts"

let socket: AttemptsSocket | null = null

const getSocket = () => {
	if (!socket) {
		socket = io(NAMESPACE, {
			autoConnect: false,
			path: SOCKET_PATH,
			transports: ["websocket"],
			withCredentials: true,
		})
	}

	return socket
}

export const connectAttemptsSocket = () => {
	const currentSocket = getSocket()

	if (!currentSocket) return null

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

export const onAttemptNew = (handler: () => void) => {
	const currentSocket = connectAttemptsSocket()
	if (!currentSocket) return () => {}

	currentSocket.on("attempts:new-attempt", handler)

	return () => {
		currentSocket.off("attempts:new-attempt", handler)
	}
}

export const onAttemptWarning = (handler: (payload: AttemptWarning) => void) => {
	const currentSocket = connectAttemptsSocket()
	if (!currentSocket) return () => {}

	currentSocket.on("attempts:warning", handler)

	return () => {
		currentSocket.off("attempts:warning", handler)
	}
}
