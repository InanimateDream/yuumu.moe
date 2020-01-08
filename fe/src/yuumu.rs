mod comp;
mod index;
mod router;
mod header;
mod body;
mod about;

pub mod prelude {
    pub use stdweb::js;
    pub use yew::prelude::*;

    use super::{
        comp,
        router,
        header,
        body,
        about,
    };
    pub use comp::{
        Term,
        Theorem,
        Incoming,
    };
    pub use router::route;
    pub use header::Header;
    pub use body::Body;
    pub use about::About;
    pub use super::{
        Tab,
        Page,
        HOME_URL,
        POST_URL,
        PROJECT_URL,
        ABOUT_URL
    };
}

pub use index::Index as App;

#[derive(Copy, Clone, PartialEq)]
pub enum Tab {
    Home,
    Post,
    Project,
    About,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Page {
    Home,
    Post,
    Project,
    About,
}

impl Into<Tab> for Page {
    fn into(self) -> Tab {
        match self {
            Page::Home => Tab::Home,
            Page::Post => Tab::Post,
            Page::Project => Tab::Project,
            Page::About => Tab::About,
        }
    }
}

pub const HOME_URL: &str = "#/";
pub const POST_URL: &str = "#/post";
pub const PROJECT_URL: &str = "#/project";
pub const ABOUT_URL: &str = "#/about";
