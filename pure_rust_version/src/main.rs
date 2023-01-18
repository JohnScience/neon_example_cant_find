use std::path::PathBuf;
use std::fs::create_dir;
use std::io::{self, BufRead};

pub(crate) const APP_NAME: &'static str = "AppName";

#[repr(u8)]
enum Outcome {
    Success = 0,
    CreateDirFailed = 1,
}

fn init_app_dir(mut app_data_path: PathBuf) -> Outcome {
    use Outcome::*;
    
    dbg!(&app_data_path);

    app_data_path.push(APP_NAME);
    if app_data_path.exists() { return Success };
    for img_fmt in ["nifti", "dicom"].iter() {
        app_data_path.push(img_fmt);
        create_dir(&app_data_path).unwrap();
        //if create_dir(&app_data_path).is_err() { return CreateDirFailed };
        for res_kind in ["images", "masks"].iter() {
            app_data_path.push(res_kind);
            if create_dir(&app_data_path).is_err() { return CreateDirFailed };
            app_data_path.pop();
        }
        app_data_path.pop();
    }
    Success
}

fn main() {
    let stdin = io::stdin();
    let app_data_path = stdin.lock().lines().next().unwrap().unwrap();
    init_app_dir(PathBuf::from(app_data_path));
}
