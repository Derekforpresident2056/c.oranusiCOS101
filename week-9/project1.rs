use std::fs::File;
use std::io::Write;

fn main() {

let mut file = File::create("project.txt").expect("create failed");

file.write_all("      LAGER    : 33 Export| Desparados  | Goldberg  | Guldeer | Heineken | Star |\n".as_bytes()).expect("not able to write");
file.write_all("                                                                       \n".as_bytes()).expect("not able to write");
file.write_all("      STOUT    : Legend   | Turbo king  | Williams  |\n".as_bytes()).expect("not able to write");
file.write_all("                                                                       \n".as_bytes()).expect("not able to write");
file.write_all(" NON ALCOHOLIC : Maltina  | Amstel malt |Malta Gold | Fayrouz |\n".as_bytes()).expect("not able to write");

}