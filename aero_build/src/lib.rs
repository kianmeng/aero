/*
 * Copyright 2021 The Aero Project Developers. See the COPYRIGHT
 * file at the top-level directory of this project.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub use target_lexicon;

use target_lexicon::Triple;

#[macro_export]
macro_rules! target_triple {
    ($triple:expr) => {
        aero_build::target_lexicon::Triple::from_str(&$triple).expect("Invalid triple literal");
    };
}

pub fn assemble_assembly_source_files(
    directory: PathBuf,
    target_triple: &Triple,
    ignore_list: &Vec<&str>,
) -> Result<(), Box<dyn Error>> {
    let read_dir = fs::read_dir(&directory)?;

    let mut nasm_build = nasm_rs::Build::new();
    let mut assembled = 0;

    for entry in read_dir {
        let entry = entry?;

        let file_name = &entry.file_name();
        let extension = Path::new(file_name).extension().and_then(OsStr::to_str);

        if entry.file_type()?.is_file() && extension == Some("asm") {
            if ignore_list.contains(&file_name.to_str().unwrap()) {
                continue;
            }

            nasm_build.file(entry.path());
            assembled += 1;
        } else if entry.file_type()?.is_dir() {
            assemble_assembly_source_files(entry.path(), target_triple, ignore_list)?;
        }
    }

    if assembled > 0 {
        nasm_build.flag("-f elf64");

        let objects = nasm_build
            .compile_objects()
            .expect("NASM build failed. Make sure you have nasm installed. https://nasm.us");

        let mut cc = cc::Build::new();

        for o in objects {
            cc.object(o);
        }

        cc.compile(directory.file_name().unwrap().to_str().unwrap());
    }

    Ok(())
}
