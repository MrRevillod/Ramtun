import type { User } from "$lib/auth/auth.dtos"

export const isStudent = (role?: User["role"]): boolean => role === "student"
export const isFunc = (role?: User["role"]): boolean => role === "func"
export const isAdmin = (role?: User["role"]): boolean => role === "admin"

export const isTeachingRole = (role?: User["role"]): boolean =>
	isFunc(role) || isAdmin(role)

export const canManageUsers = (role?: User["role"]): boolean =>
	isFunc(role) || isAdmin(role)
