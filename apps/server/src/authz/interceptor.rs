use crate::{
    auth::SessionClaims,
    authz::{AuthzAction, AuthzError, AuthzService},
    users::{User, UserRepository},
};

use std::sync::Arc;
use sword::prelude::*;

#[derive(Interceptor)]
pub struct AuthzGuard {
    authz: Arc<AuthzService>,
    users: Arc<UserRepository>,
}

impl OnRequestWithConfig<AuthzAction> for AuthzGuard {
    async fn on_request_with_config(
        &self,
        action: AuthzAction,
        mut req: Request,
    ) -> HttpInterceptorResult {
        let claims = req
            .extensions
            .get::<SessionClaims>()
            .cloned()
            .ok_or_else(JsonResponse::Unauthorized)?;

        let user = self
            .users
            .find_by_id(&claims.user_id)
            .await?
            .ok_or_else(|| AuthzError::ActorNotFound(claims.user_id.to_string()))?;

        self.authz.authorize_role(&user.role, action)?;
        req.extensions.insert::<User>(user);

        req.next().await
    }
}
