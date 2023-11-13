mod create;
mod delete;
mod get_all;
mod get_by_id;
mod update;

pub use create::create_customer_handler;
pub use delete::delete_customer_handler;
pub use get_all::customer_list_handler;
pub use get_by_id::get_customer_handler;
pub use update::update_customer_handler;
