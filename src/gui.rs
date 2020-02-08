use crate::solver::{Solution};

use gtk::{prelude::*, Inhibit, Orientation::{Horizontal, Vertical} };
use relm::{Relm, Widget, connect};
use relm_derive::{Msg, widget};
use glib::GString;

#[derive(Msg)]
pub enum TilesMsg {
    Update(usize, u32),
}

pub struct TilesModel {
    relm: Relm<TilesComp>,
}

#[widget]
impl Widget for TilesComp {
    fn init_view(&mut self) {
        for i in 0..6 {
            self.set_model_in_tile(i);
            let current_tile = self.get_tile_component(i);
            connect!(&current_tile, connect_changed(tile_comp), self.model.relm, 
                Update(i, tile_comp.get_active_id().unwrap_or(GString::from("1")).parse::<u32>().unwrap_or(1))
            );
        }
    }

    fn get_tile_component(&self, index: usize) -> &gtk::ComboBoxText {
        match index {
            0 => &self.tile_0,
            1 => &self.tile_1,
            2 => &self.tile_2,
            3 => &self.tile_3,
            4 => &self.tile_4,
            5 => &self.tile_5,
            _ => panic!("Incorrect tile index")
        }
    }

    fn set_model_in_tile(&mut self, tile_component_index: usize) {
        let tile_component = self.get_tile_component(tile_component_index);

        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 25, 50, 75, 100];

        for current in values.iter() {
            tile_component.append_text(&current.to_string().to_owned());
        }

        tile_component.set_active(Some(0));
    }

    fn model(relm: &Relm<Self>, _: ()) -> TilesModel {
        TilesModel {
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: TilesMsg) {
        match event {
            TilesMsg::Update(_index, _new_val) => {},
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,
            spacing: 4,

            #[name="tile_0"]
            gtk::ComboBoxText {
                id_column: 0,
            },

            #[name="tile_1"]
            gtk::ComboBoxText {
                id_column: 0,
            },

            #[name="tile_2"]
            gtk::ComboBoxText {
                id_column: 0,
            },

            #[name="tile_3"]
            gtk::ComboBoxText {
                id_column: 0,
            },

            #[name="tile_4"]
            gtk::ComboBoxText {
                id_column: 0,
            },

            #[name="tile_5"]
            gtk::ComboBoxText {
                id_column: 0,
            },
        }
    }
}

#[derive(Msg)]
pub enum TargetNumberMsg {
    Update(GString),
}

pub struct TargetNumberModel {
    value: u32,
}

#[widget]
impl Widget for TargetNumberComp {
    fn model() -> TargetNumberModel {
        TargetNumberModel {
            value: 1,
        }
    }

    fn update(&mut self, event: TargetNumberMsg) {
        match event {
            TargetNumberMsg::Update(new_val) => self.model.value = new_val.parse::<u32>().unwrap_or(1),
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,

            #[name="target_entry"]
            gtk::Entry {
                text: &(self.model.value.to_string()),
                hexpand: true,
                placeholder_text: Some("999"),
                alignment: 0.5,
                halign: gtk::Align::Center,
                max_length: 3,
                width_chars: 3,

                activate(comp) => TargetNumberMsg::Update(comp.get_text().unwrap_or(GString::from(""))),
                focus_out_event(comp, _focus_event) => (TargetNumberMsg::Update(comp.get_text().unwrap_or(GString::from(""))), Inhibit(false)),
            },

        }
    }
}

pub struct SolutionModel {

}

#[derive(Msg)]
pub enum SolutionMsg {

}

#[widget]
impl Widget for SolutionComponent {
    fn init_view(&mut self) {
        self.current_solution.get_buffer().unwrap().set_text("\n\n\n\n");
    }

    fn model() -> SolutionModel {
        SolutionModel {}
    }

    fn update(&mut self, msg: SolutionMsg) {

    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Box {
                orientation: Horizontal,
                spacing: 4,
                homogeneous: true,
                gtk::Button {
                    label: "|<"
                },
                gtk::Button {
                    label: "<"
                },
                gtk::Button {
                    label: ">"
                },
                gtk::Button {
                    label: ">|"
                },
            },
            #[name="current_solution"]
            gtk::TextView {
                editable: false,

            }
        }
    }
}

pub struct AppModel {
    tiles: [u32; 6],
}

#[derive(Msg)]
pub enum AppMsg {
    Quit,
    Solve,
    UpdateTile(usize, u32),
}

use self::TilesMsg::*;
use self::AppMsg::*;

#[widget]
impl Widget for Win {
    fn model() -> AppModel {
        AppModel {
            tiles: [0; 6],
        }
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            AppMsg::Quit => gtk::main_quit(),
            AppMsg::Solve => self.solve(),
            AppMsg::UpdateTile(index, new_val) => {
                self.model.tiles[index] = new_val;
            },
        }
    }

    fn solve(&mut self) {
        /////////////////////////////////////
        println!("{:?}", self.model.tiles);
        /////////////////////////////////////
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                spacing: 4,

                #[name="tiles"]
                TilesComp {
                    Update(index, new_val) => UpdateTile(index, new_val),
                },
                
                #[name="target"]
                TargetNumberComp {},

                gtk::Button {
                    label: "Solve",
                    hexpand: true,
                    clicked(_) => Solve,
                },

                SolutionComponent {},
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

pub fn run() {
    Win::run(()).expect("Failed to build window.");
}
