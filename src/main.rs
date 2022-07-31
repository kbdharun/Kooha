#![warn(
    rust_2018_idioms,
    clippy::items_after_statements,
    clippy::needless_pass_by_value,
    clippy::explicit_iter_loop,
    clippy::semicolon_if_nothing_returned,
    clippy::match_wildcard_for_single_variants,
    clippy::inefficient_to_string,
    clippy::map_unwrap_or,
    clippy::implicit_clone,
    clippy::struct_excessive_bools,
    clippy::trivially_copy_pass_by_ref,
    clippy::unreadable_literal,
    clippy::if_not_else,
    clippy::doc_markdown,
    clippy::unused_async,
    clippy::default_trait_access
)]
#![allow(
    clippy::new_without_default,
    clippy::format_push_string // TODO remove once gettext-rs fixes macro issues
)]

mod application;
mod area_selector;
mod backend;
mod config;
mod data_types;
mod error;
mod pactl;
mod settings;
mod utils;
mod window;

use gettextrs::{gettext, LocaleCategory};
use gtk::{gio, glib};

use self::application::Application;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn main() {
    pretty_env_logger::init();

    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain.");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain.");

    glib::set_application_name(&gettext("Kooha"));

    gst::init().expect("Unable to start gstreamer.");
    gtk::init().expect("Unable to start GTK4.");

    gstgif::plugin_register_static().expect("Failed to register gif plugin.");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file.");
    gio::resources_register(&res);

    let app = Application::new();
    app.run();
}
