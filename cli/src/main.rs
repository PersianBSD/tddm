use pddm_core::disk::os::windows::{list_disks, list_disks_wmi};
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("pddm")
        .version("0.1")
        .author("PersianBSD")
        .about("Partition & Disk Data Manager")
        .arg_required_else_help(true)
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List available physical disks")
                .action(ArgAction::SetTrue), // ✅ Flag بدون مقدار
        )
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .help("Show disk detailed info")
                .requires("disk")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("partition")
                .short('p')
                .long("partition")
                .help("Show partition info")
                .requires("disk")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("disk")
                .long("disk")
                .value_name("INDEX")
                .help("Specify disk index (e.g., 0, 1, 2...)")
                .num_args(1),
        )
        .get_matches();

    if matches.get_flag("list") {
        println!("🧾 Listing disks...");
        // call list_disks()
    }

    if matches.get_flag("info") {
        let idx = matches.get_one::<String>("disk").unwrap();
        println!("ℹ️ Disk info for index: {}", idx);
        // call info(idx)
    }

    if matches.get_flag("partition") {
        let idx = matches.get_one::<String>("disk").unwrap();
        println!("📦 Partition info for disk: {}", idx);
        // call partition_info(idx)
    }
}
