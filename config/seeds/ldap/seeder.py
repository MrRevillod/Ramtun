#!/usr/bin/env python3

from __future__ import annotations

import csv
from dataclasses import dataclass
from enum import IntEnum
from pathlib import Path

BASE_DN = "dc=inf,dc=uct,dc=cl"
PASSWORD = "password"


class Group(IntEnum):
    STUDENT = 500
    FUNC = 600


@dataclass(slots=True)
class User:
    uid: str
    names: str
    paternal_surname: str
    maternal_surname: str
    email: str
    group: Group
    uid_number: int
    password: str

    @property
    def cn(self) -> str:
        surname = self.paternal_surname
        if self.maternal_surname:
            surname += f" {self.maternal_surname}"
        return f"{self.names} {surname}".strip()

    @property
    def sn(self) -> str:
        if self.maternal_surname:
            return f"{self.paternal_surname} {self.maternal_surname}"
        return self.paternal_surname

    @property
    def given_name(self) -> str:
        return self.names

    @property
    def home(self) -> str:
        return f"/home/users/{self.uid}"


def render(user: User) -> str:
    return f"""dn: uid={user.uid},{BASE_DN}
objectClass: top
objectClass: inetOrgPerson
objectClass: posixAccount
objectClass: shadowAccount
uid: {user.uid}
uidNumber: {user.uid_number}
gidNumber: {user.group.value}
gecos: {user.uid}
cn: {user.cn}
sn: {user.sn}
givenName: {user.given_name}
mail: {user.email}
loginShell: /bin/bash
homeDirectory: {user.home}
userPassword: {user.password}
"""


def parse_group(value: str) -> Group:
    value = value.strip()

    if value in ("student", "500"):
        return Group.STUDENT

    if value in ("func", "600"):
        return Group.FUNC

    raise ValueError(f"Unknown group: {value}")


def load_users(path: Path) -> list[User]:
    users: list[User] = []

    with path.open(newline="", encoding="utf8") as file:
        reader = csv.DictReader(file)

        uid_number = 1000

        for row in reader:
            password = row.get("password", "").strip() or PASSWORD

            users.append(
                User(
                    uid=row["uid"],
                    names=row["names"],
                    paternal_surname=row["paternal_surname"],
                    maternal_surname=row["maternal_surname"],
                    email=row["email"],
                    group=parse_group(row["group"]),
                    uid_number=uid_number,
                    password=password,
                )
            )

            uid_number += 1

    return users


def main() -> None:
    users = load_users(Path("users.csv"))

    output = "\n".join(render(user) for user in users)

    Path("users.ldif").write_text(output, encoding="utf8")

    print(f"Generated {len(users)} LDAP users.")


if __name__ == "__main__":
    main()
