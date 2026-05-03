import type { User } from "$lib/auth/types"

export const isStudent = (role: User["role"] | undefined): boolean =>
	role === "student"

export const isAssistant = (role: User["role"] | undefined): boolean =>
	role === "assistant"

export const isFunc = (role: User["role"] | undefined): boolean => role === "func"

export const isAdmin = (role: User["role"] | undefined): boolean => role === "admin"

export const isTeachingRole = (role: User["role"] | undefined): boolean =>
	isAssistant(role) || isFunc(role) || isAdmin(role)

export const canManageUsers = (role: User["role"] | undefined): boolean =>
	isFunc(role) || isAdmin(role)
