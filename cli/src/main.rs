use clap::Command;
use pddm_core::disk::local::LocalDiskProvider;
use pddm_core::disk::provider::DiskProvider;

fn main() {
    let matches = Command::new("pddm")
        .version("0.1.0")
        .about("مدیر پارتیشن و دیسک")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("لیست دیسک‌های متصل")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("list", _)) => {
            let provider = LocalDiskProvider;
            let disks = provider.list_disks();

            println!("📦 دیسک‌های شناسایی‌شده:");
            for d in disks {
                println!(
                    "  - {} ({} GB) {}",
                    d.name,
                    d.size_gb,
                    if d.is_removable { "[Removable]" } else { "" }
                );
            }
        }
        _ => unreachable!(),
    }
}
