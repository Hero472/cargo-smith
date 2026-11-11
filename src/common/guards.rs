//! Role-based access control guards for Actix Web handlers.

use actix_web::{FromRequest, HttpRequest, dev::Payload, HttpMessage};
use futures::future::{ready, Ready};
use std::marker::PhantomData;
use serde_json::Value;
use crate::auth::claims::Claims;
use crate::common::errors::AppError;
use crate::common::roles::RequiredRole;


/// A type that specifies a role requirement for `AuthGuard`.
pub trait RoleRequirement {
    /// Returns `true` if the given role string satisfies the requirement.
    fn is_satisfied_by(role_str: &str) -> bool;

    /// Human-readable description of the requirement, used in error messages.
    fn requirement() -> &'static str;
}

// --- Single Role (existing RequiredRole markers) ---

impl<T: RequiredRole> RoleRequirement for T {
    fn is_satisfied_by(role_str: &str) -> bool {
        role_str == T::ROLE
    }

    fn requirement() -> &'static str {
        T::ROLE
    }
}

// --- AnyOf for multiple roles ---

/// Combines multiple role requirements: satisfies if *any* of them match.
///
/// # Example
/// ```ignore
/// type RequireStaff = AuthGuard<AnyOf<(Admin, User)>>;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AnyOf<T>(PhantomData<T>);

impl<T: TupleOfRoleRequirements> RoleRequirement for AnyOf<T> {
    fn is_satisfied_by(role_str: &str) -> bool {
        T::any_satisfies(role_str)
    }

    fn requirement() -> &'static str {
        "one of the allowed roles"
    }
}

/// Helper trait for combining multiple role requirements (used by [`AnyOf`]).
pub trait TupleOfRoleRequirements {
    fn any_satisfies(role_str: &str) -> bool;
}

// Implement for tuples of length 1 to 5 (expand as needed)
macro_rules! impl_tuple_roles {
    ($($T:ident),*) => {
        impl<$($T: RoleRequirement),*> TupleOfRoleRequirements for ($($T,)*) {
            fn any_satisfies(role_str: &str) -> bool {
                false $(|| $T::is_satisfied_by(role_str))*
            }
        }
    };
}

impl_tuple_roles!(T1);
impl_tuple_roles!(T1, T2);
impl_tuple_roles!(T1, T2, T3);
impl_tuple_roles!(T1, T2, T3, T4);
impl_tuple_roles!(T1, T2, T3, T4, T5);
impl_tuple_roles!(T1, T2, T3, T4, T5, T6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Authenticated;

impl RoleRequirement for Authenticated {
    fn is_satisfied_by(_role_str: &str) -> bool {
        true
    }

    fn requirement() -> &'static str {
        "valid authentication (any role accepted)"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HasRole;

impl RoleRequirement for HasRole {
    fn is_satisfied_by(role_str: &str) -> bool {
        !role_str.is_empty()
    }

    fn requirement() -> &'static str {
        "any non-empty role"
    }
}

// --- GuardClaims trait ---

/// Provides access to the authenticated user's claims inside a handler.
pub trait GuardClaims {
    fn claims(&self) -> &Claims<Value>;
    fn user_id(&self) -> &str {
        &self.claims().sub
    }
    fn username(&self) -> &str {
        self.claims()
            .data
            .get("username")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
    }
    fn role_str(&self) -> Option<&str> {
        self.claims().data.get("role").and_then(|v| v.as_str())
    }

    /// Returns `true` if the authenticated user has the given role.
    fn has_role(&self, role: &str) -> bool {
        self.role_str().map(|r| r == role).unwrap_or(false)
    }

    /// Convenience check for the `admin` role.
    fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    /// Convenience check for the `user` role.
    fn is_user(&self) -> bool {
        self.has_role("user")
    }

    /// Convenience check for the `guest` role.
    fn is_guest(&self) -> bool {
        self.has_role("guest")
    }
}

/// Guard that enforces a role requirement on an Actix Web handler.
///
/// Use the type aliases [`RequireAuth`], [`RequireAdmin`], [`RequireUser`],
/// [`RequireGuest`], or [`RequireAnyRole`] for common cases, or
/// `AuthGuard::<YourMarker>` for custom roles.
pub struct AuthGuard<R: RoleRequirement = Authenticated> {
    pub claims: Claims<Value>,
    _marker: PhantomData<R>,
}

impl<R: RoleRequirement> GuardClaims for AuthGuard<R> {
    fn claims(&self) -> &Claims<Value> {
        &self.claims
    }
}

impl<R: RoleRequirement> FromRequest for AuthGuard<R> {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = match req.extensions().get::<Claims<Value>>().cloned() {
            Some(claims) => {
                let role_str = claims.data
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if R::is_satisfied_by(role_str) {
                    Ok(AuthGuard {
                        claims,
                        _marker: PhantomData,
                    })
                } else {
                    Err(AppError::Forbidden(format!(
                        "insufficient permissions: requires '{}', found '{}'",
                        R::requirement(),
                        if role_str.is_empty() { "<none>" } else { role_str }
                    )))
                }
            }
            None => Err(AppError::Unauthorized("missing authentication claims")),
        };
        ready(result)
    }
}

use crate::common::roles::{Admin, User, Guest};

/// Guard that only checks for a valid JWT (any role or no role).
pub type RequireAuth = AuthGuard<Authenticated>;

/// Guard that requires the user to have *some* role (any non-empty value).
pub type RequireAnyRole = AuthGuard<HasRole>;

/// Guard requiring the `admin` role.
pub type RequireAdmin = AuthGuard<Admin>;
/// Guard requiring the `user` role.
pub type RequireUser = AuthGuard<User>;
/// Guard requiring the `guest` role.
pub type RequireGuest = AuthGuard<Guest>;

/// Generates type aliases for auth guards from a custom role enum.
#[macro_export]
macro_rules! create_auth_aliases {
    ($enum_ty:ty, $($marker:ty => $alias:ident),* $(,)?) => {
        $(
            pub type $alias = $crate::guards::AuthGuard<$marker>;
        )*
    };
}

#[cfg(test)]
mod tests {
    use crate::{common::roles::{Admin, DefaultRoles, Guest, Role, User}, define_custom_roles};

    use super::*;
    use actix_web::test::TestRequest;
    use serde_json::json;

    fn create_request_with_claims(claims: Claims<Value>) -> HttpRequest {
        let req = TestRequest::default().to_http_request();
        req.extensions_mut().insert(claims);
        req
    }

    fn create_claims(role: &str) -> Claims<Value> {
        Claims {
            sub: "123".to_string(),
            exp: 9999999999,
            data: json!({
                "username": "testuser",
                "role": role
            }),
            iat: 999999999
        }
    }

    #[test]
    fn default_roles_as_str() {
        assert_eq!(DefaultRoles::Admin.as_str(), "admin");
        assert_eq!(DefaultRoles::User.as_str(), "user");
        assert_eq!(DefaultRoles::Guest.as_str(), "guest");
    }

    #[test]
    fn default_required_role_constants() {
        assert_eq!(Admin::ROLE, "admin");
        assert_eq!(User::ROLE, "user");
        assert_eq!(Guest::ROLE, "guest");
    }

    define_custom_roles! {
        TestRoles {
            Editor => EditorRole,
            Viewer => ViewerRole,
        }
    }

    #[test]
    fn custom_roles_macro_generates_correctly() {
        use TestRoles::*;
        assert_eq!(Editor.as_str(), "editor");
        assert_eq!(Viewer.as_str(), "viewer");
        assert_eq!(EditorRole::ROLE, "editor");
        assert_eq!(ViewerRole::ROLE, "viewer");
    }

    #[actix_web::test]
    async fn auth_guard_single_role_succeeds() {
        let claims = create_claims("admin");
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = AuthGuard::<Admin>::from_request(&req, &mut payload).await;
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn auth_guard_single_role_fails_wrong_role() {
        let claims = create_claims("user");
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = AuthGuard::<Admin>::from_request(&req, &mut payload).await;
        match result {
            Err(AppError::Forbidden(msg)) => assert!(msg.starts_with("insufficient permissions")),
            _ => panic!("Expected Forbidden"),
        }
    }

    #[actix_web::test]
    async fn auth_guard_anyof_succeeds_with_either_role() {
        let claims = create_claims("user");
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = AuthGuard::<AnyOf<(Admin, User)>>::from_request(&req, &mut payload).await;
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn auth_guard_anyof_fails_with_neither_role() {
        let claims = create_claims("guest");
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = AuthGuard::<AnyOf<(Admin, User)>>::from_request(&req, &mut payload).await;
        match result {
            Err(AppError::Forbidden(msg)) => assert!(msg.starts_with("insufficient permissions")),
            _ => panic!("Expected Forbidden"),
        }
    }

    #[actix_web::test]
    async fn auth_guard_norole_succeeds_with_any_role() {
        let claims = create_claims("guest");
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = AuthGuard::<Authenticated>::from_request(&req, &mut payload).await;
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn require_auth_success_with_missing_role() {
        let mut claims = create_claims("admin");
        claims.data = json!({ "username": "test" }); // no role
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = RequireAuth::from_request(&req, &mut payload).await;
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn guard_fails_when_no_claims_in_extensions() {
        let (req, mut payload) = TestRequest::default().to_http_parts();

        let result = AuthGuard::<Admin>::from_request(&req, &mut payload).await;
        match result {
            Err(AppError::Unauthorized(msg)) => assert_eq!(msg, "missing authentication claims"),
            _ => panic!("Expected Unauthorized"),
        }
    }

    #[actix_web::test]
    async fn require_any_role_fails_with_missing_role() {
        let mut claims = create_claims("admin");
        claims.data = json!({ "username": "test" }); // no role
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = RequireAnyRole::from_request(&req, &mut payload).await;
        match result {
            Err(AppError::Forbidden(msg)) => assert!(msg.starts_with("insufficient permissions")),
            _ => panic!("Expected Forbidden due to missing role"),
        }
    }

    #[actix_web::test]
    async fn guard_fails_when_role_field_missing() {
        let mut claims = create_claims("admin");
        claims.data = json!({ "username": "test" }); // no "role"
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = RequireAdmin::from_request(&req, &mut payload).await;
        match result {
            Err(AppError::Forbidden(msg)) => assert!(msg.starts_with("insufficient permissions")),
            _ => panic!("Expected Forbidden due to missing role"),
        }
    }

    #[actix_web::test]
    async fn guard_fails_when_role_not_deserializable() {
        let mut claims = create_claims("admin");
        claims.data = json!({ "username": "test", "role": 123 }); // not a string
        let (req, mut payload) = TestRequest::default().to_http_parts();
        req.extensions_mut().insert(claims);

        let result = RequireAdmin::from_request(&req, &mut payload).await;
        match result {
            Err(AppError::Forbidden(msg)) => assert!(msg.starts_with("insufficient permissions")),
            _ => panic!("Expected Forbidden due to invalid role type"),
        }
    }

    #[actix_web::test]
    async fn guard_claims_trait_methods() {
        let claims = create_claims("user");
        let guard = AuthGuard::<User> {
            claims: claims.clone(),
            _marker: PhantomData,
        };
        assert_eq!(guard.user_id(), "123");
        assert_eq!(guard.username(), "testuser");
        assert_eq!(guard.role_str(), Some("user"));
    }
}