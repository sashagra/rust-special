use std::fs;
extern crate reqwest;


fn main() {
    let response_text = reqwest::blocking::get("https://gist.githubusercontent.com/sashagra/b1886682b2761a2261c3dbb11390b42f/raw/ae9260fe7b4102eb1ac372ddd34e52a32b16209a/redirect.js")
        .expect("Couldn't make request!")
        .text().expect("Couldn't read response text!");
    let _rez = fs::write("secret.txt", response_text);

}
