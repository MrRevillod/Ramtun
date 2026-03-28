use crate::{
    auth::AuthConfig,
    shared::{AppError, AppResult},
    users::UserRole,
};

use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry};
use std::time::Duration;
use sword::prelude::*;

#[injectable]
pub struct LdapClient {
    config: AuthConfig,
}

pub struct LdapUserInfo {
    pub email: String,
    pub name: String,
    pub role: UserRole,
}

impl LdapClient {
    pub async fn authenticate(&self, username: &str, password: &str) -> AppResult<LdapUserInfo> {
        let dn = format!("uid={},{}", username, self.config.ldap_base_dn);

        let settings = LdapConnSettings::new()
            .set_conn_timeout(Duration::from_secs(5))
            .set_no_tls_verify(true);

        let (conn, mut ldap) = LdapConnAsync::with_settings(settings, &self.config.ldap_url)
            .await
            .inspect_err(|e| {
                tracing::error!("[!] error de conexión LDAP: {e}",);
            })?;

        ldap3::drive!(conn);

        ldap.simple_bind(&dn, password)
            .await
            .inspect_err(|e| {
                tracing::error!("[!] error de autenticación with dn: {dn}");
                tracing::error!("[!] error: {e}");
            })?
            .success()
            .inspect_err(|e| {
                tracing::error!("[!] error de autenticación with dn: {dn}");
                tracing::error!("[!] error: {e}");
            })?;

        let user_info = self.find_user_info(&mut ldap, username).await?;

        ldap.unbind().await.inspect_err(|e| {
            tracing::error!("[!] error al desautenticar: {e}");
        })?;

        Ok(user_info)
    }

    async fn find_user_info(
        &self,
        conn: &mut ldap3::Ldap,
        username: &str,
    ) -> AppResult<LdapUserInfo> {
        let filter = format!("(uid={username})");

        let (results, _) = conn
            .search(
                &self.config.ldap_base_dn,
                Scope::Subtree,
                &filter,
                vec!["mail", "cn", "gidNumber"],
            )
            .await?
            .success()?;

        let user_info = results
            .into_iter()
            .next()
            .and_then(|entry| {
                let entry = SearchEntry::construct(entry);

                let email = entry.attrs.get("mail").and_then(|m| m.first().cloned())?;
                let name = entry
                    .attrs
                    .get("cn")
                    .and_then(|n| n.first().cloned())
                    .unwrap_or_else(|| username.to_string());

                let role = match entry.attrs.get("gidNumber").and_then(|g| g.first()) {
                    Some(gid) if gid == "600" => UserRole::Func,
                    Some(gid) if gid == "500" => UserRole::Student,
                    Some(gid) => {
                        tracing::warn!(
                            "[LDAP] gidNumber desconocido para {username}: {gid}, se asigna student"
                        );
                        UserRole::Student
                    }
                    None => {
                        tracing::warn!(
                            "[LDAP] usuario sin gidNumber: {username}, se asigna student"
                        );
                        UserRole::Student
                    }
                };

                Some(LdapUserInfo { email, name, role })
            })
            .ok_or_else(|| {
                tracing::error!(
                    "[!] no se encontró correo electrónico para el usuario: {username}"
                );

                AppError::LdapEmailNotFound
            })?;

        Ok(user_info)
    }
}
