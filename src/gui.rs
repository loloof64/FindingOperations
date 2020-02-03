use crate::solver::{Solution};

use gtk::{prelude::*, Inhibit, Orientation::Horizontal, Orientation::Vertical};
use relm::{Widget};
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
            TilesMsg::Update(index, newVal) => self.model.tiles[index] = newVal,
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,
            spacing: 10,

            gtk::Entry {
                editing_done => {
                    
                }
            },

            gtk::Entry {
                
            },

            gtk::Entry {
                
            },

            gtk::Entry {
                
            },

            gtk::Entry {
                
            },

            gtk::Entry {
                
            }
        }
    }
}

pub struct AppModel {

}

#[derive(Msg)]
pub enum AppMsg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> AppModel {
        AppModel {}
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical
            },

            delete_event(_, _) => (AppMsg::Quit, Inhibit(false)),
        }
    }
}

pub fn run() {
    Win::run(()).expect("Failed to build window.");
}
