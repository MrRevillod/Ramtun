use crate::{
    auth::AuthConfig,
    shared::{AppError, AppResult},
    users::UserRole,
};

use ldap3::{Ldap, LdapConnAsync as LdapConn, LdapConnSettings, Scope, SearchEntry};
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
        let mut ldap = self.ldap_connect().await?;

        let admin_dn = format!(
            "{},{}",
            self.config.ldap_admin_user, self.config.ldap_base_dn
        );

        tracing::debug!("[*] Autenticando como admin: {}", admin_dn);

        ldap.simple_bind(&admin_dn, &self.config.ldap_admin_password)
            .await
            .inspect_err(|e| {
                tracing::error!("[!] Error de conexión durante bind admin: {e}");
            })?
            .success()
            .inspect_err(|e| {
                tracing::error!("[!] Error de autenticación como admin LDAP: {e}");
            })?;

        let user_dn = self
            .find_user_dn(&mut ldap, username)
            .await
            .inspect_err(|e| {
                tracing::error!("[!] No se pudo encontrar usuario {}: {}", username, e);
            })?;

        ldap.unbind().await.inspect_err(|e| {
            tracing::warn!("[!] Error al desautenticar admin: {e}");
        })?;

        let mut ldap = self.ldap_connect().await?;

        ldap.simple_bind(&user_dn, password)
            .await
            .inspect_err(|e| {
                tracing::error!("[!] Error de conexión durante bind de usuario: {e}");
            })?
            .success()
            .inspect_err(|e| {
                tracing::warn!("[!] Autenticación fallida para usuario {}: {e}", username);
            })?;

        tracing::info!("[✓] Usuario {} autenticado exitosamente", username);

        let user_info = self.find_user_info(&mut ldap, &user_dn, username).await?;

        self.ldap_unbind(&mut ldap).await;

        Ok(user_info)
    }

    async fn find_user_dn(&self, conn: &mut Ldap, username: &str) -> AppResult<String> {
        let filter = format!("(uid={username})");

        let (results, _) = conn
            .search(
                &self.config.ldap_base_dn,
                Scope::Subtree,
                &filter,
                vec!["dn"],
            )
            .await?
            .success()?;

        if results.is_empty() {
            tracing::error!("[!] Usuario no encontrado en LDAP: {}", username);
            return Err(AppError::LdapUsernameNotFound(username.to_string()));
        }

        let dn = results
            .iter()
            .map(|entry| SearchEntry::construct(entry.clone()).dn)
            .collect::<Vec<String>>()
            .get(0)
            .ok_or_else(|| {
                tracing::error!("[!] No se pudo extraer DN para el usuario: {}", username);
                AppError::LdapUsernameNotFound(username.to_string())
            })?
            .to_owned();

        tracing::debug!("[✓] DN encontrado: {}", dn);

        Ok(dn)
    }

    async fn find_user_info(
        &self,
        conn: &mut Ldap,
        user_dn: &str,
        username: &str,
    ) -> AppResult<LdapUserInfo> {
        let filter = "(|(objectClass=inetOrgPerson)(objectClass=posixAccount))";

        tracing::debug!("[*] Buscando atributos del usuario: {}", user_dn);

        let (results, _) = conn
            .search(
                user_dn,
                Scope::Base,
                filter,
                vec!["mail", "cn", "gidNumber"],
            )
            .await?
            .success()?;

        let entry = results.into_iter().next().ok_or_else(|| {
            tracing::error!(
                "[!] no se encontró correo electrónico para el usuario: {username}"
            );

            AppError::LdapEmailNotFound
        })?;

        let entry = SearchEntry::construct(entry);

        let email = entry
            .attrs
            .get("mail")
            .and_then(|mail| mail.first().cloned())
            .ok_or_else(|| {
                tracing::error!(
                    "[!] no se encontró correo electrónico para el usuario: {username}"
                );

                AppError::LdapEmailNotFound
            })?;

        let name = entry
            .attrs
            .get("cn")
            .and_then(|full_name| full_name.first().cloned())
            .unwrap_or_else(|| username.to_string());

        let role = match entry.attrs.get("gidNumber").and_then(|g| g.first()) {
            Some(gid) if gid == "600" => {
                tracing::debug!("[*] Usuario {} es Func (gid=600)", username);
                UserRole::Func
            }
            Some(gid) if gid == "500" => {
                tracing::debug!("[*] Usuario {} es Student (gid=500)", username);
                UserRole::Student
            }
            Some(gid) => {
                tracing::warn!(
                    "[LDAP] gidNumber desconocido para {}: {}, se asigna Student",
                    username,
                    gid
                );
                UserRole::Student
            }
            None => {
                tracing::warn!(
                    "[LDAP] usuario sin gidNumber: {}, se asigna Student",
                    username
                );
                UserRole::Student
            }
        };

        let user_info = LdapUserInfo { email, name, role };

        tracing::info!(
            "[✓] Información obtenida para {}: email={}, role={:?}",
            username,
            user_info.email,
            user_info.role
        );

        Ok(user_info)
    }

    async fn ldap_connect(&self) -> AppResult<Ldap> {
        let settings = LdapConnSettings::new()
            .set_conn_timeout(Duration::from_secs(5))
            .set_no_tls_verify(true);

        let (conn, ldap) = LdapConn::with_settings(settings, &self.config.ldap_url)
            .await
            .inspect_err(|e| {
                tracing::error!("[!] Error de conexión LDAP: {e}");
            })?;

        ldap3::drive!(conn);

        Ok(ldap)
    }

    async fn ldap_unbind(&self, conn: &mut Ldap) {
        if let Err(e) = conn.unbind().await {
            tracing::warn!("[!] Error al desautenticar LDAP: {e}");
        }
    }
}
