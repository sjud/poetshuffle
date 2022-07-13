use uuid::Uuid;
use bincode::{Encode,Decode};
use strum::{AsRefStr,EnumString};
#[derive(Encode,Decode,Clone,Debug,PartialEq)]
pub struct UploadWsBinary{
    pub headers:UploadHeaders,
    pub file:Vec<u8>,
}
#[derive(Encode,Decode,Clone,Copy,Debug,PartialEq)]
pub struct UploadHeaders{
    pub file_ty:FileType,
    pub table_cat:TableCategory,
    #[bincode(with_serde)]
    pub uuid:Uuid,
}
#[derive(Encode,Decode,Clone,Debug,Copy,PartialEq,AsRefStr,EnumString)]
pub enum FileType{
    #[strum(ascii_case_insensitive)]
    Audio,
    #[strum(ascii_case_insensitive)]
    Transcript
}
impl FileType{
    pub fn header_name() -> &'static str {
        "x-file-type"
    }
}
#[derive(Encode,Decode,Clone,Debug,Copy,PartialEq,AsRefStr,EnumString)]
pub enum TableCategory{
    #[strum(ascii_case_insensitive)]
    Intros,
    #[strum(ascii_case_insensitive)]
    Poems,
    #[strum(ascii_case_insensitive)]
    Banters,
}

impl TableCategory{
    pub fn header_name() -> &'static str {
        "x-category"
    }
}
