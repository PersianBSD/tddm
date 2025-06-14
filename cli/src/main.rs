use pddm_core::disk::os::windows::{list_disks, list_disks_wmi};
use clap::{Arg, Command};
use pddm_core::partition::provider::list_partitions;


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
                .help("List available physical disks"),
        )
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .help("Show disk detailed info")
                .requires("disk"),
        )
        .arg(
            Arg::new("partition")
                .short('p')
                .long("partition")
                .help("Show partition info")
                .requires("disk"),
        )
        .arg(
            Arg::new("disk")
                .long("disk")
                .value_name("DISK")
                .num_args(1)
                .value_name("INDEX")
                .help("Specify disk index (e.g., 0, 1, 2...)"),
        )
        .get_matches();

    if matches.contains_id("list") {
        // call list_disks()
    }

    if matches.contains_id("info") {
        let idx = matches.get_one::<String>("disk").unwrap();
        // call info(idx)
    }

    if matches.contains_id("partition") {
        let idx = matches.get_one::<String>("disk").unwrap();
        // call partition_info(idx)
    }
}

