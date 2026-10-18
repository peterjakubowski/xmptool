use std::env;
use xmpkit::{XmpFile};
use xmptool::get_all_persons_shown;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let mut file = XmpFile::new();
    let _ = file.open(file_path);

    if let Some(meta) = file.get_xmp() {
        if let Some(persons_shown) = get_all_persons_shown(meta) {
            println!("{:#?}", persons_shown);
        } else {
            println!("No persons shown!")
        }
    } else {
        println!("No xmp here!");
    }    
}
