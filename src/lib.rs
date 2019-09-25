use std::fmt::Display;
use simplelog::ConfigBuilder;
use std::collections::HashMap;

pub mod io;

pub trait Turn {}

pub struct Tale {
    attributes : HashMap<String, String>,
    start : Chapter,
    config : Config,
}

impl std::fmt::Display for Tale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"{:?}", self.attributes)
    }
}

pub struct Config {}

impl Config {
    pub(crate) fn new() -> Config {
        Config {}
    }
}

pub struct Chapter {
    shallow : bool,
    path : Option<String>,
    pages : Vec<Page>
}

impl Chapter {
    pub(crate) fn new() -> Chapter {
        Chapter {
            shallow: true,
            path : None,
            pages : Vec::new()
        }
    }
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