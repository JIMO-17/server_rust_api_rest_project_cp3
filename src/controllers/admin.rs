mod create;
mod delete;
mod get_all;
mod get_by_id;
mod update;

pub use create::create_admin_handler;
pub use delete::delete_admin_handler;
pub use get_all::admin_list_handler;
pub use get_by_id::get_admin_handler;
pub use update::update_admin_handler;
