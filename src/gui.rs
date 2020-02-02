use crate::solver::{Solution};

use gtk::{prelude::*, Inhibit, Orientation::Horizontal};
use relm::{Widget};
use relm_derive::{Msg, widget};

#[derive(Msg)]
pub enum TilesMsg {
    Update(u8, u32),
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
            }

            gtk::Entry {
                
            }

            gtk::Entry {
                
            }

            gtk::Entry {
                
            }

            gtk::Entry {
                
            }

            gtk::Entry {
                
            }
        }
    }
}

pub fn run() {
    Win::run(()).expect("Failed to build window.");
}
