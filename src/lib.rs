#![doc = include_str!("../README.md")]
#![deny(
	missing_docs,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates,
	unused_import_braces,
	unused_qualifications
)]
#![warn(missing_debug_implementations, dead_code, clippy::unwrap_used, clippy::expect_used)]

use std::{env, str::FromStr};

pub use dotenv;
use http::uri::Uri;
pub use sqlx_database_tester_macros::test;

const DATABASE_ENV_VAR: &str = "DATABASE_URL";

#[doc(hidden)]
/// Create a UUID based database name with optional prefix from the database
/// specified in the connection string
pub fn derive_db_name(uri: &str) -> http::Result<String> {
	let target_database_uri_parts = Uri::from_str(uri)?.into_parts();
	let random_part = uuid::Uuid::new_v4().to_simple().to_string();

	let database_name_prefix = target_database_uri_parts
		.path_and_query
		.map(|paq| paq.path().replace('/', "").trim().to_owned())
		.filter(|p| !p.is_empty());
	Ok(if let Some(prefix) = database_name_prefix {
		format!("{}_{}", prefix, random_part)
	} else {
		random_part
	})
}

#[doc(hidden)]
/// Retrieve the database uri for a specific database
pub fn get_target_database_uri(uri: &str, db_name: &str) -> http::Result<String> {
	let target_database_uri_parts = Uri::from_str(uri)?.into_parts();
	let builder = Uri::builder();
	let builder = if let Some(scheme) = target_database_uri_parts.scheme {
		builder.scheme(scheme)
	} else {
		builder
	};
	let builder = if let Some(authority) = target_database_uri_parts.authority {
		builder.authority(authority)
	} else {
		builder
	};
	Ok(builder.path_and_query(format!("/{}", db_name)).build()?.to_string())
}

#[doc(hidden)]
/// Retrieve database_uri from the env variable, panics if it's not available
pub fn get_database_uri() -> String {
	env::var(DATABASE_ENV_VAR)
		.unwrap_or_else(|_| panic!("The env variable {} needs to be set", DATABASE_ENV_VAR))
}
