use crate::file_type::FileType;

pub fn get_file_type(file_name: &str) -> FileType {
    if file_name.ends_with(".txt") {
        FileType::Text
    } else if file_name.ends_with(".jpg") || file_name.ends_with(".png") {
        FileType::Image
    } else if file_name.ends_with(".mp4") {
        FileType::Video
    } else {
        FileType::Text
    }
}
