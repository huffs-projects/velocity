pub mod home;
pub mod apps;
pub mod recent;
pub mod settings;

pub use home::render_home;
pub use apps::render_apps;
pub use recent::render_recent;
pub use settings::render_settings;
