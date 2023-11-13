mod create;
mod delete;
mod get_all;
mod get_by_id;
mod update;

pub use create::create_product_handler;
pub use delete::delete_product_handler;
pub use get_all::product_list_handler;
pub use get_by_id::get_product_handler;
pub use update::update_product_handler;
