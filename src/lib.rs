use std::fmt::Display;

pub mod io;

pub trait Turn {}

pub struct Story {

}

pub struct Chapter {
    pages : Vec<Page>
}

impl Turn for Chapter {}

pub struct Page {
    next : Vec<Box<dyn Turn>>,
    logic : PageLogic,
    content : Box<dyn PageContent>
}

impl Turn for Page {}

pub enum PageLogic {
    Next,
}

trait PageContent : Display {}