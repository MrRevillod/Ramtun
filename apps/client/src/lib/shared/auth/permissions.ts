import type { User } from "$lib/auth/types"

export const isStudent = (role?: User["role"]): boolean => role === "student"
export const isAssistant = (role?: User["role"]): boolean => role === "assistant"
export const isFunc = (role?: User["role"]): boolean => role === "func"
export const isAdmin = (role?: User["role"]): boolean => role === "admin"

export const isTeachingRole = (role?: User["role"]): boolean =>
	isAssistant(role) || isFunc(role) || isAdmin(role)

export const canManageUsers = (role?: User["role"]): boolean =>
	isFunc(role) || isAdmin(role)
