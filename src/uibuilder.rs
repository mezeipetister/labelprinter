use gtk::{self, prelude::*};

#[derive(Clone, Debug)]
pub struct UI {
    pub builder: gtk::Builder,
}

impl UI {
    pub fn new() -> UI {
        // The order here is important because some ui file depends on others

        let builder = gtk::Builder::new();

        builder
            .add_from_resource("org/mezeipetister/labelprinting/design.ui")
            .expect("Can't load ui file: design.ui");

        UI { builder }
    }
}
