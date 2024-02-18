use std::time::Duration;
use async_channel;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow, Button, SearchBar, Image};
use gtk::glib::clone;
use gtk::gio::Settings;

const APP_ID: &str = "org.gtk_rs.MainEventLoop1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    let (sender, recver) = async_channel::bounded(1);
    button.connect_clicked(move |_| {
        let sender = sender.clone();
        // GUI is blocked for 5 seconds after the button is pressed
        // gio::spawn_blocking(move || {
            
        // });
        glib::spawn_future_local(clone!(@strong sender => async move {
            sender.send_blocking(false).expect("The channel need to be open");
            let five_seconds = Duration::from_secs(5);
            glib::timeout_future_seconds(5).await;
            sender.send_blocking(true).expect("The channel need to be open");
        }));
    });

    glib::spawn_future_local(clone!(@weak button => async move {
        while let Ok(enable_button) = recver.recv().await {
            button.set_sensitive(enable_button);
        }
    }));

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}