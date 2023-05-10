extern crate exif;
extern crate chrono;
use iced::Color;
use std::path::Path;
use crate::get_dirlistr;
pub fn listpressr (dir_value: String) -> (Color, String, String) {
     let errstring: String;
     let mut new_dirlist: String = " ".to_string();
     let colorx : Color;
     let dir_path = Path::new(&dir_value);
     let (errcd, errstr, newliststr) = get_dirlistr(dir_path.to_path_buf());
     if errcd == 0 {
         new_dirlist = newliststr;
         errstring = "got directory".to_string();
         colorx = Color::from([0.0, 0.0, 0.0]);
     } else {
         errstring = errstr.to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
     }
    (colorx, errstring, new_dirlist)
}

