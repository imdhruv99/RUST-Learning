mod file;
mod file_operations;
mod file_type;
mod printable;

use crate::file::File;
use crate::file_operations::get_file_type;
use crate::file_type::FileType;
use crate::printable::Printable;

fn main() {
    let text_file = File::new("document.txt");
    let image_file = File::new("image.jpg");

    text_file.print();
    image_file.print();

    let text_file_type = get_file_type("document.txt");
    let image_file_type = get_file_type("image.jpg");

    match text_file_type {
        FileType::Text => println!("Text file detected."),
        FileType::Image => println!("Image file detected."),
        FileType::Video => println!("Video file detected."),
    }

    match image_file_type {
        FileType::Text => println!("Text file detected."),
        FileType::Image => println!("Image file detected."),
        FileType::Video => println!("Video file detected."),
    }
}
