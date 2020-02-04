use crate::solver::{Solution};

use gtk::{prelude::*, Inhibit, Orientation::Horizontal, Orientation::Vertical, CellRendererText};
use relm::{Relm, Widget, Component, init, connect};
use relm_derive::{Msg, widget};

#[derive(Msg)]
pub enum TilesMsg {
    Update(usize, u32),
}

pub struct TilesModel {
    tiles: [u32; 6],
    relm: Relm<TilesComp>,
}

#[widget]
impl Widget for TilesComp {
    fn init_view(&mut self) {
        for i in 0..6 {
            self.set_model_in_tile(i);
        }
    }

    fn set_model_in_tile(&mut self, tile_component_index: isize) {
        let list_store = gtk::ListStore::new(&[u32::static_type()]);
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 25, 50, 75, 100];

        for current in values.iter() {
            list_store.insert_with_values(None, &[0], &[current]);
        }

        let tile_component = match tile_component_index {
            0 => &self.tile_0,
            1 => &self.tile_1,
            2 => &self.tile_2,
            3 => &self.tile_3,
            4 => &self.tile_4,
            5 => &self.tile_5,
            _ => panic!("Incorrect tile index")
        };

        tile_component.set_model(Some(&list_store));
        let renderer = CellRendererText::new();
        tile_component.pack_start(&renderer, true);
        tile_component.add_attribute(&renderer, "text", 0);
    }

    fn model(relm: &Relm<Self>, _: ()) -> TilesModel {
        TilesModel {
            tiles: [0; 6],
            relm: relm.clone(),
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

            #[name="tile_0"]
            gtk::ComboBox {},

            #[name="tile_1"]
            gtk::ComboBox {},

            #[name="tile_2"]
            gtk::ComboBox {},

            #[name="tile_3"]
            gtk::ComboBox {},

            #[name="tile_4"]
            gtk::ComboBox {},

            #[name="tile_5"]
            gtk::ComboBox {},
        }
    }
}

#[derive(Msg)]
pub enum TargetNumberMsg {
    Update(u32),
}

pub struct TargetNumberModel {
    value: u32,
}

#[widget]
impl Widget for TargetNumberComp {
    fn model() -> TargetNumberModel {
        TargetNumberModel {
            value: 0,
        }
    }

    fn update(&mut self, event: TargetNumberMsg) {
        match event {
            TargetNumberMsg::Update(new_val) => self.model.value = new_val,
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,
            gtk::Label {
                justify: gtk::Justification::Center,
                text: &self.model.value.to_string(),
                hexpand: true,
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
    
}

#[derive(Msg)]
pub enum AppMsg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> AppModel {
        AppModel {

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
                spacing: 4,

                #[name="tiles"]
                TilesComp {},
                
                #[name="target"]
                TargetNumberComp {},

                gtk::Button {
                    label: "Solve",
                    hexpand: true,
                },

                SolutionComponent {},
            },

            delete_event(_, _) => (AppMsg::Quit, Inhibit(false)),
        }
    }
}

pub fn run() {
    Win::run(()).expect("Failed to build window.");
}
