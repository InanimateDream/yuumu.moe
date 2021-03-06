use super::prelude::*;
use stdweb::web::{
    window,
    IEventTarget,
};
use stdweb::web::event::HashChangeEvent;

pub fn route(callback: Callback<Page>) {
    let dispatch = move || {
        let hash = window().location().unwrap().hash().unwrap();
        match hash.as_str() {
            HOME_URL => callback.emit(Page::Home),
            POST_URL => callback.emit(Page::Post),
            PROJECT_URL => callback.emit(Page::Project),
            ABOUT_URL => callback.emit(Page::About),
            _ => {}
        }
    };
    dispatch();
    window().add_event_listener(move |_: HashChangeEvent| {
        dispatch();
    });
}

