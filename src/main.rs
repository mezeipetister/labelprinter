extern crate cairo;
extern crate chrono;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate pango;
extern crate pangocairo;

mod app;
mod static_resources;
mod uibuilder;
mod window_state;

use chrono::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{AboutDialog, ApplicationWindow, Builder, Button, Entry, Window};

use std::env::args;

#[derive(Clone)]
struct Data {
    pname: Entry,
    pinfo: Entry,
    name: Entry,
    zip: Entry,
    city: Entry,
    street: Entry,
    taxnumber: Entry,
}

trait FoldText {
    fn fold(&self) -> String;
}

impl FoldText for Entry {
    fn fold(&self) -> String {
        self.get_text().unwrap().to_string()
    }
}

fn about(button: &Button, dialog: &AboutDialog) {
    if let Some(window) = button
        .get_toplevel()
        .and_then(|w| w.downcast::<Window>().ok())
    {
        dialog.set_transient_for(Some(&window));
    }

    // Since we only have once instance of this object with Glade, we only show/hide it.
    dialog.show();
    dialog.run();
    dialog.hide();
}

fn print_datal_relative(
    x: f64,
    y: f64,
    cairo: &cairo::Context,
    pango_layout: &pango::Layout,
    data: &Data,
) {
    let row_dst: f64 = 15.0;

    pango_layout.set_text("");
    cairo.move_to(x, y);
    pangocairo::functions::show_layout(&cairo, &pango_layout);
    // Draw text1
    pango_layout.set_text(&format!("{} - {}", data.pname.fold(), data.pinfo.fold()));
    cairo.rel_move_to(0.0, row_dst);
    pangocairo::functions::show_layout(&cairo, &pango_layout);

    // Draw text1
    pango_layout.set_text(&format!("{} - {}", data.name.fold(), data.zip.fold()));
    cairo.rel_move_to(0.0, row_dst + 10.0);
    pangocairo::functions::show_layout(&cairo, &pango_layout);

    //Draw text2 below text1
    pango_layout.set_text(&format!("{}, {}", data.city.fold(), data.street.fold()));
    cairo.rel_move_to(0.0, row_dst);
    pangocairo::functions::show_layout(&cairo, &pango_layout);

    pango_layout.set_text(&format!("Adószám: {}", data.taxnumber.fold()));
    cairo.rel_move_to(0.0, row_dst);
    pangocairo::functions::show_layout(&cairo, &pango_layout);

    let to_two_digit = |i: u32| -> String {
        match i < 10 {
            true => format!("0{}", i),
            false => format!("{}", i),
        }
    };

    pango_layout.set_text(&format!(
        "Dátum: {}-{}-{}",
        Utc::today().year(),
        to_two_digit(Utc::today().month()),
        to_two_digit(Utc::today().day())
    ));
    cairo.rel_move_to(0.0, row_dst);
    pangocairo::functions::show_layout(&cairo, &pango_layout);
}

fn print(window: &gtk::ApplicationWindow, data: Data) {
    let print_operation = gtk::PrintOperation::new();

    // Currently unused
    // Could be used to check whether there was a success in printing
    //let print_operation_result: gtk::PrintOperationResult;

    print_operation.connect_begin_print(move |print_operation, _| {
        // This sets the number of pages of the document.
        // You most likely will calculate this, but for this example
        // it's hardcoded as 1
        print_operation.set_n_pages(1);
        print_operation.set_unit(gtk::Unit::Mm);
        let psetup = gtk::PageSetup::new();
        psetup.set_paper_size(&gtk::PaperSize::new_from_ppd(
            "custom", "a4", 595.27, 841.88,
        ));
        print_operation.set_default_page_setup(Some(&psetup));
    });

    print_operation.connect_draw_page(move |_, print_context, _| {
        let cairo = print_context
            .get_cairo_context()
            .expect("Couldn't get cairo context");

        // This allows you to get the width of the page
        // Currently unused in this example
        // let width = print_context.get_width();

        //Initi pango and set a font
        let font_description = pango::FontDescription::from_string("sans 12");
        let pango_layout = print_context
            .create_pango_layout()
            .expect("Couldn't create pango layout");
        pango_layout.set_font_description(Option::from(&font_description));

        let top_margin: f64 = 20.0;
        let h: f64 = 201.0;
        let rows = 4;

        for row in 0..rows {
            print_datal_relative(
                0.0,
                h * (row as f64) + top_margin,
                &cairo,
                &pango_layout,
                &data,
            );
            print_datal_relative(
                300.0,
                h * (row as f64) + top_margin,
                &cairo,
                &pango_layout,
                &data,
            );
        }
    });

    //Open Print dialog setting up main window as its parent
    print_operation
        .run(gtk::PrintOperationAction::PrintDialog, Option::from(window))
        .expect("Couldn't print");
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../data/ui/design.ui");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder
        .get_object("mainwindow")
        .expect("Couldn't get window");
    window.set_application(Some(application));

    let button_about: Button = builder
        .get_object("info")
        .expect("Couldn't get info button");

    let dialog: AboutDialog = builder
        .get_object("about_dialog")
        .expect("Couldn't get dialog");
    let dc = dialog.clone();
    button_about.connect_clicked(move |x| about(x, &dialog));

    let btn_close_about: Button = builder
        .get_object("close_about")
        .expect("Couldn't get info button");

    btn_close_about.connect_clicked(move |_| dc.hide());

    let data = Data {
        pname: builder
            .get_object("d_pname")
            .expect("Couldn't get name entry"),
        pinfo: builder
            .get_object("d_pinfo")
            .expect("Couldn't get name entry"),
        name: builder
            .get_object("d_name")
            .expect("Couldn't get name entry"),
        zip: builder
            .get_object("d_zip")
            .expect("Couldn't get name entry"),
        city: builder
            .get_object("d_city")
            .expect("Couldn't get name entry"),
        street: builder
            .get_object("d_street")
            .expect("Couldn't get name entry"),
        taxnumber: builder
            .get_object("d_taxnumber")
            .expect("Couldn't get name entry"),
    };

    data.zip.set_input_purpose(gtk::InputPurpose::Digits);

    let btn_print: Button = builder
        .get_object("d_print")
        .expect("Couldn't get print button");

    let w = window.clone();
    btn_print.connect_clicked(move |_| {
        print(&w, data.clone());
        println!(
            "Name is {}, zip is {}",
            data.name.get_text().unwrap().to_string(),
            data.zip.get_text().unwrap().to_string()
        )
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.labelprinting"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
