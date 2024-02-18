use gtk4 as gtk;
use gtk4::{glib, prelude::*};
use gtk::{Image, Picture};

fn main() -> glib::ExitCode {
    let application = gtk4::Application::builder()
        .application_id("cn.prehistory.shell")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_stock_search_container() -> gtk::Box{
    let _search_contain_box = gtk::Box::new(gtk::Orientation::Vertical, 6);

    let _label = gtk::Label::default();
    _label.set_markup("<b>shellmound for STOCK research</b>");
    _label.set_valign(gtk::Align::Start);

    let _search_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let _entry = gtk::Entry::builder().valign(gtk::Align::Start).max_width_chars(10).build();
    _entry.set_width_request(100);
    let _btn = gtk::Button::builder().label("find me").build();
    _entry.set_hexpand(true);
    _search_contain_box.append(&_label);
    _search_box.append(&_entry);
    _search_box.append(&_btn);

    _search_contain_box.append(&_search_box);

    // load image to show
    let image = Image::new();

    let _pic = Picture::for_filename(".runtime/stock.png");

    _search_contain_box.append(&_pic);

    return _search_contain_box;
}

fn build_search_bar() -> gtk::SearchBar {
    let _search_bar = gtk::SearchBar::builder()
        .valign(gtk::Align::Start)
        .build();
    let _entry = gtk::Entry::new();
    _entry.set_hexpand(true);
    _search_bar.set_child(Some(&_entry));
    _search_bar
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("shellmound"));
    window.set_default_size(800, 480);

    let _search_bar = build_search_bar();
    let _container = build_stock_search_container();
    window.set_child(Some(&_container));

    window.present();
}