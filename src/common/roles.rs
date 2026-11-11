use serde::{Serialize, Deserialize};
use strum::{VariantNames, EnumString, Display, AsRefStr};
use std::fmt::Debug;

/// A role that can be attached to a user's JWT claims.
///
/// Implement this for any type that represents an application role.
pub trait Role:
    Serialize + for<'de> Deserialize<'de> + PartialEq + Clone + Send + Sync + Debug + 'static
{
    /// Returns the role's string representation (e.g. `"admin"`).
    fn as_str(&self) -> &str;
}

/// A marker type that declares a single required role string.
///
/// Used by [`AuthGuard`] to enforce role-based access control.
/// Implement this for zero-sized marker structs such as [`Admin`].
pub trait RequiredRole {
    /// The role string this marker requires (e.g. `"admin"`).
    const ROLE: &'static str;
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Display, AsRefStr, EnumString, VariantNames)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// The built-in roles shipped with cargo-smith.
///
/// Variants serialize as lowercase strings (`"admin"`, `"user"`, `"guest"`).
/// Use [`Admin`], [`User`], or [`Guest`] as guard markers, and this enum
/// for the `role` field in JWT claims.
pub enum DefaultRoles {
    #[default]
    User,
    Admin,
    Guest,
}

impl Role for DefaultRoles {
    fn as_str(&self) -> &str {
        self.as_ref() // Provided by AsRefStr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Admin;
impl RequiredRole for Admin { 
    const ROLE: &'static str = "admin"; 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct User;
impl RequiredRole for User { 
    const ROLE: &'static str = "user"; 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Guest;
impl RequiredRole for Guest { 
    const ROLE: &'static str = "guest"; 
}

/// Compile-time macro that lowercases an ASCII string literal.
///
/// Used internally by [`define_custom_roles!`] to derive role strings from
/// variant names. Not intended for direct use.
#[macro_export]
macro_rules! ascii_lowercase {
    ($s:expr) => {{
        const INPUT: &str = $s;
        const OUTPUT: &str = {
            const fn lower_byte(b: u8) -> u8 {
                if b.is_ascii_uppercase() { b + 32 } else { b }
            }

            const fn convert<const N: usize>(input: &[u8]) -> [u8; N] {
                let mut out = [0; N];
                let mut i = 0;
                while i < N {
                    // SAFETY: The slice is guaranteed to have length N.
                    out[i] = lower_byte(unsafe { *input.as_ptr().add(i) });
                    i += 1;
                }
                out
            }

            // Store the array in an associated constant to give it 'static lifetime.
            struct Helper<const N: usize>;
            impl<const N: usize> Helper<N> {
                const ARRAY: [u8; N] = convert::<N>(INPUT.as_bytes());
            }

            // SAFETY: We only modified ASCII uppercase letters, so output is valid UTF-8.
            // The reference to Helper::ARRAY is truly 'static.
            unsafe { std::str::from_utf8_unchecked(&Helper::<{ INPUT.len() }>::ARRAY) }
        };
        OUTPUT
    }};
}

/// Defines a custom set of application roles.
///
/// Generates an enum implementing [`Role`] and marker structs implementing
/// [`RequiredRole`] for each variant. Variant names are PascalCase; role
/// strings are their lowercase form.
///
/// # Example
/// ```
/// use cargo_smith::define_custom_roles;
///
/// define_custom_roles! {
///     AppRoles {
///         Editor => EditorRole,
///         Viewer => ViewerRole,
///     }
/// }
///
/// // Now use `RequireEditor`-style guards via `AuthGuard::<EditorRole>`
/// ```
#[macro_export]
macro_rules! define_custom_roles {
    ($name:ident { $($variant:ident => $marker:ident),* $(,)? }) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq,
            serde::Serialize, serde::Deserialize,
            strum::EnumString, strum::VariantNames, strum::Display, strum::AsRefStr
        )]
        #[serde(rename_all = "lowercase")]
        #[strum(serialize_all = "lowercase")]
        pub enum $name {
            $($variant),*
        }

        impl $crate::common::roles::Role for $name {
            fn as_str(&self) -> &str {
                self.as_ref()
            }
        }

        $(
            pub struct $marker;
            impl $crate::common::roles::RequiredRole for $marker {
                const ROLE: &'static str = $crate::ascii_lowercase!(
                    <$name as strum::VariantNames>::VARIANTS[$name::$variant as usize]
                );
            }
        )*
    };
}