#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod ncda;
mod errors;
mod ws;

fn main() {
    // for _i in 0..40_000_000 {
    //     ncda::check("cb32752361d");
    // }

    ws::launch_ws(None);
}
