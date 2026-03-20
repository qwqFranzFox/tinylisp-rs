extern crate alloc;
pub use alloc::string::String;
pub use alloc::string::ToString;
pub use alloc::sync::Arc;
pub use alloc::vec;
pub use alloc::vec::Vec;
pub use spin::Lazy;
pub use spin::RwLock;

use rp235x_hal as hal;
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"Blinky Example"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];
