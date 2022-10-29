use glib::types::Type;
use gtk::prelude::*;
use gtk::{
    builders::MenuBarBuilder, Application, ApplicationWindow, Box, Button, ComboBox, ComboBoxText,
    Label, MenuItem, PackDirection,
};

fn main() {
    gtk::init().expect("Failed GTK");
    let app = Application::builder().application_id("Window").build();
    app.connect_activate(|app| {
        // main window.
        //------------------------------------------------------------
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(820)
            .default_height(600)
            .title("Main Menu")
            .resizable(true)
            .show_menubar(true)
            .window_position(gtk::WindowPosition::Center)
            .border_width(50)
            .build();
        // container
        //------------------------------------------------------------
        let container = Box::new(gtk::Orientation::Vertical, 10);
        // menu bar
        //------------------------------------------------------------
        let menu_bar = MenuBarBuilder::new()
            .name("MenuBar")
            .child_pack_direction(PackDirection::Ltr)
            .border_width(0)
            .margin_start(5)
            .margin_end(15)
            .margin_top(5)
            .margin_bottom(15)
            .has_tooltip(true)
            .tooltip_text("Hello")
            .visible(true)
            .build();
        // menu item
        //------------------------------------------------------------
        let item1 = MenuItem::with_label("Item1");
        let item2 = MenuItem::with_label("Item2");
        let item3 = MenuItem::with_label("Item3");
        menu_bar.add(&item1);
        menu_bar.add(&item2);
        menu_bar.add(&item3);
        container.add(&menu_bar);
        //button 1
        //------------------------------------------------------------
        let button1 = Button::new();
        let label1 = Label::with_mnemonic("Press me!");
        button1.add(&label1);
        container.add(&button1);
        //combobox - creation
        //------------------------------------------------------------
        let cb_button = ComboBox::new();
        let label = Label::with_mnemonic("Combo box");
        cb_button.add(&label);
        container.add(&cb_button);
        //textbox with entry - creation
        //------------------------------------------------------------
        let new_entry = gtk::Entry::new();
        container.add(&new_entry);
        new_entry.connect_activate(|x| println!("{}", x.text()));
        //combobox with entry - creation
        //------------------------------------------------------------
        let cb_button_entry = ComboBox::with_entry();
        container.add(&cb_button_entry);
        //combobox with entry & model- creation
        //------------------------------------------------------------
        //let model = gtk::ListStore::new(&[Type::U8, Type::STRING]);
        let model = gtk::ListStore::new(&[Type::STRING]);
        let entries = &["jj", "sam", "anne"];
        //let summary: String = String::from("Item");
        for (_, entry) in entries.iter().enumerate() {
            //for _ in 0..10 {
            model.set(&model.append(), &[(0, &entry)]);
            //model.set(&model.append(), &[(0, &"item")]);
            //model.set(&model.append(), &[(0, &summary.to_owned())]);
            println!("{:?}", model.to_value());
        }
        println!("model: {}", &model);
        let cb_button_model_entry = ComboBox::with_model_and_entry(&model);
        container.add(&cb_button_model_entry);
        //combobox text - creation - for GTK3
        //------------------------------------------------------------
        let cb_button_text = ComboBoxText::builder().tooltip_text("Combobox").build();
        cb_button_text.append_text("item1");
        cb_button_text.append_text("item2");
        cb_button_text.append_text("item3");
        container.add(&cb_button_text);
        //button 2
        //------------------------------------------------------------
        let button = Button::with_label("Click me!");
        let label = Label::new(Some("No click yet!"));
        button.connect_clicked(|_| {
            eprintln!("you've clicked, so we can run something on the back!");
        });
        let label_click = label.clone();
        button.connect_clicked(move |_| {
            label_click.set_label("Button clicked!");
        });
        container.add(&button);
        container.add(&label);
        //window
        //------------------------------------------------------------
        window.add(&container);
        window.show_all();
    });

    app.run();
}
