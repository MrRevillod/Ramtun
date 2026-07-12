use crate::auth::{AuthConfig, SessionCheck};
use crate::authz::{AuthzAction, AuthzGuard};
use crate::shared::RequestExt;
use crate::users::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = Controller::Web, path = "/users")]
pub struct UsersController {
	users: Arc<UsersService>,
	auth_config: AuthConfig,
}

impl UsersController {
	#[get("/me")]
	#[interceptor(SessionCheck)]
	#[doc = "Get the current authenticated user's information"]
	pub async fn get_me(&self, req: Request) -> WebResult<UserView> {
		let claims = req.claims().ok_or_else(JsonResponse::Unauthorized)?;
		let user = self.users.find_by_id(&claims.user_id).await?;

		Ok(user)
	}

	#[get("/")]
	#[interceptor(SessionCheck)]
	#[interceptor(AuthzGuard, config = AuthzAction::UserListAdmin)]
	#[doc = "List all users in system (admin only) with all details."]
	pub async fn list_users(&self, req: Request) -> WebResult<Vec<UserView>> {
		let query = req.query::<SearchUsersQuery>()?.unwrap_or_default();
		let users = self.users.list_users(query).await?;

		Ok(users)
	}

	#[get("/collaborator-candidates")]
	#[interceptor(SessionCheck)]
	#[interceptor(AuthzGuard, config = AuthzAction::UserListCollaboratorCandidates)]
	#[doc = "List users eligible to be added as course members"]
	pub async fn list_collaborator_candidates(&self, req: Request) -> WebResult<Vec<UserView>> {
		let query = req.query::<SearchUsersQuery>()?.unwrap_or_default();
		let users = self.users.list_collaborator_candidates(query).await?;

		Ok(users)
	}

	#[patch("/{userId}/role")]
	#[interceptor(SessionCheck)]
	#[interceptor(AuthzGuard, config = AuthzAction::UserManageRole)]
	#[doc = "Update a user's global role (func/admin executable only)"]
	pub async fn set_user_role(&self, req: Request) -> WebResult {
		let user_id = req.param::<UserId>("userId")?;
		let input = req.body_validator::<UpdateUserRoleDto>()?;

		self.users.update_role(&user_id, input).await?;

		Ok(JsonResponse::Ok())
	}

	#[patch("/ldap-password-sync")]
	pub async fn patch_user_password(&self, req: Request) -> WebResult {
		let input = req.body_validator::<UpdatePasswordDto>()?;

		let provided_api_key = req
			.headers()
			.get("Authorization")
			.and_then(|v| v.to_str().ok())
			.ok_or_else(JsonResponse::Unauthorized)?;

		if provided_api_key != self.auth_config.password_sync_api_key {
			return Err(JsonResponse::Unauthorized());
		}

		self.users.update_password(input).await?;

		Ok(JsonResponse::Ok())
	}
}
