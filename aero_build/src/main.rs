use fs_extra::dir;
use fs_extra::dir::CopyOptions;

use structopt::StructOpt;

use std::env;
use std::fs;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitStatus};

/// The cargo executable. This constant uses the `CARGO` environment variable to
/// also support non-standard cargo versions.
const CARGO: &str = env!("CARGO");

const BUNDLED_DIR: &str = "bundled";
const BUILD_DIR: &str = "build";

mod bootloader;
mod bundled;

/// Build the kernel by using `cargo build` with the cargo config defined
/// in the `src\.cargo\config.toml` file.
fn build_kernel(target: Option<String>, bootloader: AeroBootloader) {
    println!("INFO: Building kernel");

    let mut kernel_build_cmd = Command::new(CARGO);

    kernel_build_cmd.current_dir("src");

    kernel_build_cmd.arg("build");
    kernel_build_cmd.arg("--package").arg("aero_kernel");

    match bootloader {
        AeroBootloader::AeroBoot => {}

        AeroBootloader::Limine => {
            kernel_build_cmd.args(&["--feature", "limine"]);
        }

        AeroBootloader::Tomato => {
            kernel_build_cmd.args(&["--feature", "tomato"]);
        }

        AeroBootloader::Multiboot2 => {
            kernel_build_cmd.args(&["--feature", "multiboot2"]);
        }
    }

    // Use the specified target. By default it will build for x86_64-aero_os
    if let Some(target) = target {
        kernel_build_cmd
            .arg("--target")
            .arg(format!("./.cargo/{}.json", target));
    }

    if !kernel_build_cmd
        .status()
        .expect(&format!("Failed to run {:#?}", kernel_build_cmd))
        .success()
    {
        panic!("Failed to build the kernel")
    }
}

/// Runs Aero in qemu with UEFI as its default mode. By default it will
/// mount the build directory as a FAT partition instead of creating a seperate
/// `.fat` file. Check out [AeroBuild] for configuration settings about this.
fn run_qemu(argv: Vec<String>) -> ExitStatus {
    let mut qemu_run_cmd = Command::new("qemu-system-x86_64");

    qemu_run_cmd.args(argv);

    // Set up OVMF.
    qemu_run_cmd
        .arg("-drive")
        .arg("if=pflash,format=raw,file=bundled/ovmf/OVMF_CODE-pure-efi.fd");
    qemu_run_cmd
        .arg("-drive")
        .arg("if=pflash,format=raw,file=bundled/ovmf/OVMF_VARS-pure-efi.fd");
    qemu_run_cmd
        .arg("-bios")
        .arg("bundled/ovmf/OVMF-pure-efi.fd");

    qemu_run_cmd.arg("-machine").arg("type=q35");
    qemu_run_cmd.arg("-cpu").arg("qemu64");
    qemu_run_cmd.arg("-smp").arg("2");
    qemu_run_cmd.arg("-m").arg("512M");

    qemu_run_cmd
        .arg("-drive")
        .arg("format=raw,file=fat:rw:build/"); // Mounts the build directory as a FAT partition

    qemu_run_cmd
        .status()
        .expect(&format!("Failed to run {:#?}", qemu_run_cmd))
}

/// Build Aero's main webiste including its docs.
fn build_web() -> Result<(), Box<dyn Error>> {
    let mut docs_build_cmd = Command::new(CARGO);

    docs_build_cmd.current_dir("src");
    docs_build_cmd.arg("doc");

    // Generate the docs.
    if !docs_build_cmd
        .status()
        .expect(&format!("Failed to run {:#?}", docs_build_cmd))
        .success()
    {
        panic!("Failed to build docs")
    }

    let cargo_output_dir = Path::new("src")
        .join("target")
        .join("x86_64-aero_os")
        .join("doc");

    let build_dir = Path::new("web").join("build");

    // Create the docs build directory.
    fs::create_dir_all(&build_dir)?;

    let mut cp_options = CopyOptions::new();
    cp_options.overwrite = true;

    // First move each file from the web/* directory to web/build/*
    for entry in fs::read_dir("web")? {
        let item = entry?;

        if item.file_type()?.is_file() {
            fs::copy(item.path(), build_dir.join(item.file_name()))?;
        }
    }

    // Now move all of the generated doc files by cargo to web/build/.
    dir::copy(cargo_output_dir, &build_dir, &cp_options)?;

    Ok(())
}

/// Packages all of the files by creating the build directory and copying
/// the `aero.elf` and the `aero_boot.efi` files to the build directory and
/// creating the `startup.nsh` file.
fn package_files() -> Result<(), Box<dyn Error>> {
    // Create the build directory.
    fs::create_dir_all("build/efi/boot")?;
    fs::create_dir_all("build/efi/kernel")?;

    fs::copy(
        "src/target/x86_64-aero_os/debug/aero_kernel",
        "build/efi/kernel/aero.elf",
    )?;

    fs::copy(
        "src/target/x86_64-unknown-uefi/debug/aero_boot.efi",
        "build/efi/boot/aero_boot.efi",
    )?;

    // Create the `startup.nsh` file.
    let mut startup_nsh = File::create("build/startup.nsh")?;
    startup_nsh.write_all(br"\efi\boot\aero_boot.EFI")?;

    Ok(())
}

#[derive(Debug)]
enum AeroBootloader {
    AeroBoot,
    Limine,
    Tomato,
    Multiboot2,
}

impl From<Option<String>> for AeroBootloader {
    fn from(boot: Option<String>) -> Self {
        if let Some(boot) = boot {
            match boot.as_ref() {
                "aero" => Self::AeroBoot,
                "limine" => Self::Limine,
                "tomato" => Self::Tomato,
                "multiboot2" => Self::Multiboot2,
                _ => panic!("Invalid or unsupported bootloader {}", boot),
            }
        } else {
            Self::AeroBoot
        }
    }
}

#[derive(Debug, StructOpt)]
enum AeroBuildCommand {
    /// Build and run Aero in qemu.
    Run {
        #[structopt(long)]
        target: Option<String>,

        #[structopt(long)]
        chainloader: Option<String>,
        bootloader: Option<String>,

        /// Extra command line arguments passed to qemu.
        #[structopt(last = true)]
        qemu_args: Vec<String>,
    },

    Build {
        bootloader: Option<String>,
        target: Option<String>,
    },

    /// Update all of the OVMF files required for UEFI and bootloader prebuilts.
    Update {
        bootloader: Option<String>,
    },

    Web,
}

#[derive(Debug, StructOpt)]
struct AeroBuild {
    #[structopt(subcommand)]
    command: Option<AeroBuildCommand>,
}

#[tokio::main]
async fn main() {
    let aero_build = AeroBuild::from_args();

    match aero_build.command {
        Some(command) => match command {
            AeroBuildCommand::Run {
                mut qemu_args,
                target,
                bootloader,
                chainloader,
            } => {
                let bootloader = AeroBootloader::from(bootloader);

                bundled::download_ovmf_prebuilt().await.unwrap();

                match bootloader {
                    AeroBootloader::AeroBoot => bootloader::build_bootloader(),
                    AeroBootloader::Limine => bundled::download_limine_prebuilt().await.unwrap(),
                    AeroBootloader::Tomato => {}
                    AeroBootloader::Multiboot2 => {}
                }

                build_kernel(target, bootloader);
                package_files().unwrap();

                if let Some(chainloader) = chainloader {
                    qemu_args.push("-drive".into());
                    qemu_args.push(format!("format=raw,file={}", chainloader));
                }

                if !run_qemu(qemu_args).success() {
                    panic!("Failed to run qemu");
                }
            }

            AeroBuildCommand::Build { bootloader, target } => {
                let bootloader = AeroBootloader::from(bootloader);

                bundled::download_ovmf_prebuilt().await.unwrap();

                match bootloader {
                    AeroBootloader::AeroBoot => bootloader::build_bootloader(),
                    AeroBootloader::Limine => bundled::download_limine_prebuilt().await.unwrap(),
                    AeroBootloader::Tomato => {}
                    AeroBootloader::Multiboot2 => {}
                }

                build_kernel(target, bootloader);
                package_files().unwrap();
            }

            AeroBuildCommand::Update { bootloader } => {
                let bootloader = AeroBootloader::from(bootloader);

                bundled::update_ovmf()
                    .await
                    .expect("Failed tp update OVMF files");

                if let AeroBootloader::Limine = bootloader {
                    bundled::update_limine()
                        .await
                        .expect("Failed to update limine prebuilt files");
                }
            }

            AeroBuildCommand::Web => build_web().unwrap(),
        },

        None => {}
    }
}
