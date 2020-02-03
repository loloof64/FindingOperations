use crate::solver::{Solution};

use gtk::{prelude::*, Inhibit, Orientation::Horizontal, Orientation::Vertical};
use relm::{Widget, Component, init};
use relm_derive::{Msg, widget};

#[derive(Msg)]
pub enum TilesMsg {
    Update(usize, u32),
}

pub struct TilesModel {
    tiles: [u32; 6],
}

#[widget]
impl Widget for TilesComp {
    fn model() -> TilesModel {
        TilesModel {
            tiles: [0; 6],
        }
    }

    fn update(&mut self, event: TilesMsg) {
        match event {
            TilesMsg::Update(index, new_val) => self.model.tiles[index] = new_val,
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,
            spacing: 4,

            gtk::Entry {
                max_length: 3,
                max_width_chars: 3,
                width_chars: 3,
                placeholder_text: Some("000"),
                editing_done => {
                    
                }
            },

            gtk::Entry {
                max_length: 3,
                max_width_chars: 3,
                width_chars: 3,
                placeholder_text: Some("000"),
            },

            gtk::Entry {
                max_length: 3,
                max_width_chars: 3,
                width_chars: 3,
                placeholder_text: Some("000"),
            },

            gtk::Entry {
                max_length: 3,
                max_width_chars: 3,
                width_chars: 3,
                placeholder_text: Some("000"),
            },

            gtk::Entry {
                max_length: 3,
                max_width_chars: 3,
                width_chars: 3,
                placeholder_text: Some("000"),
            },

            gtk::Entry {
                max_length: 3,
                max_width_chars: 3,
                width_chars: 3,
                placeholder_text: Some("000"),
            },

            gtk::Entry {
                max_length: 3,
                max_width_chars: 3,
                width_chars: 3,
                placeholder_text: Some("000"),
            }
        }
    }
}

pub struct AppModel {
    tiles_comp: Component<TilesComp>,
}

#[derive(Msg)]
pub enum AppMsg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> AppModel {
        let tiles = init::<TilesComp>(()).expect("Failed to build tiles component");
        AppModel {
            tiles_comp: tiles,
        }
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,

                center_widget: Some(self.model.tiles_comp.widget()),
            },

            delete_event(_, _) => (AppMsg::Quit, Inhibit(false)),
        }
    }
}

pub fn run() {
    Win::run(()).expect("Failed to build window.");
}
