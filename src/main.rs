extern crate mac_notification_sys;
extern crate open;
use mac_notification_sys::*;

fn main() {
    //println!("Hello, world!");
    let bundle = get_bundle_identifier_or_default("iterm");
    set_application(&bundle).unwrap();
    /*
    send_notification("danger",
                      &Some("will Robinson"),
                      "Run away as fast as you can,",
                      &Some("Blow"))
        .unwrap();
   */ 
    send_notification("Now",
                      &None,
                      "Look at you brower",
                      &Some("Submarine"))
        .unwrap();

    if open::that("https://rust-lang.org").is_ok() {
        println!("Look at you brower!");
    }
}
