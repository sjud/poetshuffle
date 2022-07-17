mod read_button;
pub use read_button::*;
mod text_reader;
pub use text_reader::*;

use uuid::Uuid;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount};
use shared::{FileType, TableCategory};
use crate::types::auth_context::AuthContext;
use crate::types::msg_context::{MsgContext, new_red_msg_with_std_duration};
use crate::types::transcript_context::{TranscriptActions, TranscriptContext};
