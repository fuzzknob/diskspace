use disks::get_disk_info;
use out::print_result;

mod disks;
mod out;

fn main() {
    let disks = get_disk_info();
    match print_result(disks) {
        Ok(_) => (),
        Err(_) => println!("There was an error printing disk space"),
    }
}
