pub mod accept_invitation;
pub mod admin;
pub mod login;
pub mod login_register;
pub mod register;
pub mod validate_registration;

use crate::queries::validation::*;
use crate::services::network::post_graphql;
use crate::services::utility::map_graphql_errors_to_string;
use uuid::Uuid;
use yew::prelude::*;

