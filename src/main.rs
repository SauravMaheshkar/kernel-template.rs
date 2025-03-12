/// Main build script for building a bootable disk image
///
/// Environment variables for this script are generated in the [build script](https://github.com/SauravMaheshkar/kernel-template.rs/blob/main/build.rs).
/// All arguments to `qemu` must be defined in this script. For example, to enable serial output you'd normally pass `-serial stdio` as extra arguments to `qemu`.
/// In the build script, you'd need to add
///
/// ```rust
/// let mut cmd = std::process::Command::new("qemu-system-x86_64");
/// cmd.arg("-serial").arg("stdio");
/// ```
///
/// This script then builds the `kernel` member crate which contains the main chunk of the OS.
fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    // choose whether to start the UEFI or BIOS image
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }

    // enable serial output
    cmd.arg("-serial").arg("stdio");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
