#![no_std]
#![no_main]

#![feature(alloc)]

extern crate alloc;

extern crate uefi;
extern crate uefi_alloc;
extern crate uefi_services;

use alloc::string::String;
use alloc::vec::Vec;

use uefi::table::{SystemTable, Boot};
use uefi::proto::media::{
    fs::SimpleFileSystem,
    file::*,
};
use uefi::data_types::Align;
use uefi::table::boot::{
    SearchType,
};

use core::fmt::Write;

fn crawl_tree (system_table: &SystemTable<Boot>, dir: &mut Directory, depth: u8) {
  let mut buf = Vec::<u8>::with_capacity(FileInfo::alignment() * 50);
  loop {
    match dir.read_entry(&mut buf[..]).map_err(|err| err.split()) {
      Ok(ret) => {
        if let Some(f) = ret.log() {
          let filename = String::from_utf16_lossy(f.file_name().to_u16_slice());
          if filename.as_str() == "." || filename.as_str() == ".." {
            continue;
          }

          for _ in 0..depth {
            let _ = system_table.stdout().write_str("    ");
          };
          let _ = system_table.stdout().write_str(filename.as_str());

          match dir.handle().open(filename.as_str(), FileMode::Read, FileAttribute::READ_ONLY) {
            Ok(f) => {
              if let Ok(c) = f.log().into_type() {
                match c.log() {
                  FileType::Dir(mut d) => {
                    let _ = system_table.stdout().write_str("/\n");
                    crawl_tree(system_table, &mut d, depth+1);
                  }
                  FileType::Regular(_) => {
                    let _ = system_table.stdout().write_str("\n");
                  }
                }
              }
            }
            Err(_) => {}
          }
        } else {
          break;
        }
      }
      Err((_, Some(new_size))) => {
        buf.extend((0..new_size - buf.len()).map(|_| 0));
      }
      Err((status, None)) => panic!("Can't read root dir status: {:?}", status),
    };
  };
}

#[no_mangle]
pub extern "win64" fn efi_main(_handle: uefi::Handle, system_table: SystemTable<Boot>)  {
    unsafe{ uefi_alloc::init(system_table.boot_services()) };

    let size = system_table
        .boot_services()
        .locate_handle(SearchType::from_proto::<SimpleFileSystem>(), None).expect("Could not locate handles")
        .unwrap();

    assert!(size > 0);

    let mut buffer = Vec::with_capacity(size);

    unsafe { buffer.set_len(size); }

    system_table.boot_services()
        .locate_handle(SearchType::from_proto::<SimpleFileSystem>(), Some(&mut buffer[..])).expect("Could not locate handle")
        .unwrap();

    let fs_handle = buffer.first().unwrap();

    let fs = system_table.boot_services()
        .handle_protocol::<SimpleFileSystem>(*fs_handle).expect("Could not handle protocol")
        .unwrap()
        .get();

    let _ = system_table.stdout().write_str("Ficheros en la particion EFI: \n");

    let mut root = unsafe { (*fs).open_volume().expect("Could not open volume").unwrap() };
    crawl_tree(&system_table, &mut root, 0);

    loop {}
}
