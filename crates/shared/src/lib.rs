use uuid::Uuid;
use bincode::{Encode,Decode};
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
#[derive(Encode,Decode,Clone,Debug,Copy,PartialEq)]
pub enum FileType{
    Audio,
    Transcript
}
#[derive(Encode,Decode,Clone,Debug,Copy,PartialEq)]
pub enum TableCategory{
    Intros,
    Poems,
    Banter,
}