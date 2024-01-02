//The most basic note card structs and enums. Notecards will be stored in the database
pub mod storage_controller;
// mod server;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NoteCardElement{
    ///Regular text
    Text(String),
    ///The address of an image
    ImageUrl(String),
    ///The address of a video
    VideoUrl(String),
    ///Just a url
    Url(String),
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BasicNoteCard{
    front:NoteCardElement,
    back:NoteCardElement,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NineByNineNoteCard{
    top_row:[NoteCardElement;3],
    mid_row:[NoteCardElement;3],
    bot_row:[NoteCardElement;3]
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NoteCardVariants{
    NineByNineNoteCard(NineByNineNoteCard),
    BasicNoteCard(BasicNoteCard),
}
