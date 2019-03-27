#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;

#[macro_use]
extern crate log;

use uefi::table::{SystemTable, Boot, Runtime};
use core::fmt::Write;

#[no_mangle]
pub extern "win64" fn efi_main(_image_handle: uefi::Handle, system_table: SystemTable<Boot>) -> ! {
    let rev = system_table.uefi_revision();
    info!("UEFI {}.{}", rev.major(), rev.minor());
    loop {}
}
