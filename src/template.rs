use super::sql::new_file_name;
use log::info;
use seahorse::Context;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn new_template(c: &Context) {
    info!("creating new template");

    let template = "create table foo;\n";

    if c.args.len() != 2 {
        println!("Path to migration or seeds and a name for the new file is required. Try something like this:\n\ndb new db/seeds add-test-data");
        std::process::exit(1);
    }

    let path = &c.args[0];
    let nfn = new_file_name(path, &c.args[1]);
    let mut file = File::create(Path::new(path).join(nfn)).expect("could not create new file");
    file.write_all(template.as_bytes())
        .expect("could not write template to new file");
}
