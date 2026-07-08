import { RoleValue } from "$lib/shared/value-objects/role.value"

import type { UserDTO } from "./dtos"

export class User {
	constructor(
		public id: string,
		public username: string,
		public name: string,
		public email: string,
		public role: RoleValue
	) {}

	public static fromDTO(dto: UserDTO): User {
		const role = RoleValue.from(dto.role) ?? RoleValue.default()
		return new User(dto.id, dto.username, dto.name, dto.email, role)
	}

	public isStudent(): boolean {
		return this.role.isStudent()
	}

	public isFunc(): boolean {
		return this.role.isFunc()
	}

	public isAdmin(): boolean {
		return this.role.isAdmin()
	}
}
