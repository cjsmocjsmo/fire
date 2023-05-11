// use glob::glob;
use std::env;
use std::fs;
use std::path::Path;
use glob::glob;

fn get_current_working_dir() -> String {
    let path = env::current_dir().unwrap();
    let dir_path = String::from(path.to_string_lossy());

    dir_path
}



// fn set_env_var(p1: String, p2: String) -> Result<(), Box<dyn std::error::Error>> {
//     env::set_var(&p1, p2);
//     let value = env::var(&p1);
//     if value.is_err() {
//         println!("Error: key not found");
//     } else {
//         println!("key is set to: {}", value.unwrap());
//     }

//     Ok(())
// }

pub struct FireDirs {
    fire_dir_nfos: String,
    fire_dir_thumbnails: String,
    fire_dir: String,

}

impl FireDirs {
    fn set_fire_dir_nfos(&self) -> bool {
        let nfo_dir = get_current_working_dir().to_string() + &self.fire_dir_nfos;
        let nfos_dir_exists = Path::new(&nfo_dir).is_dir();
        if !nfos_dir_exists {
            fs::create_dir_all(nfo_dir.clone()).unwrap();
        }

        true
    }
    fn set_fire_dir_thumbnails(&self) -> bool {
        let thumb_dir = get_current_working_dir().to_string() + &self.fire_dir_thumbnails;
        let thumb_dir_exists = Path::new(&thumb_dir).is_dir();
        if thumb_dir_exists {
            fs::create_dir_all(thumb_dir.clone()).unwrap();
        }

        true
    }
    fn set_fire_dir(&self) -> bool {
        let fire_dir = get_current_working_dir().to_string() + &self.fire_dir;
        let fire_dir_exists = Path::new(&fire_dir).is_dir();
        if fire_dir_exists {
            fs::create_dir_all(fire_dir.clone()).unwrap();
        }

        true
    }
    fn clean_nfos_dir(&self) -> u32 {
        let nfo_meta_dir_path = &self.fire_dir_nfos;
        let glob_str = nfo_meta_dir_path.to_owned() + "/*.nfo";
        let mut count = 0;
        for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
            let rm_path = e.unwrap();
            count = count + 1;
            fs::remove_file(rm_path).expect("File delete failed");
        }

        count
    }
    fn clean_thumbnail_dir(&self) -> u32 {
        let music_meta_dir_path = &self.fire_dir_thumbnails;
        let glob_str = music_meta_dir_path.to_owned() + "/*.jpg";
        let mut count = 0;
        for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
            count = count + 1;
            let rm_path = e.unwrap();
            fs::remove_file(rm_path).expect("File delete failed");
        }
        count
    }
}

pub fn create_dirs() {
    let fd = FireDirs {
        fire_dir_nfos: "/fire_dir/nfos".to_string(),
        fire_dir_thumbnails: "/fire_dir/thumbnails".to_string(),
        fire_dir: "/fire_dir".to_string(),
    };

    let cwd = get_current_working_dir().to_string();
    let cdir = cwd + &fd.fire_dir;

    let dir_exists = Path::new(&cdir).is_dir();
    if !dir_exists {
        fd.set_fire_dir();
    };
    
    let thumbnail_path = cdir.to_string() + &fd.fire_dir_thumbnails.to_string();
    let thumbnails_exist = Path::new(&thumbnail_path).is_dir();
    if !thumbnails_exist {
        fd.set_fire_dir_thumbnails();
    } else {
        fd.clean_thumbnail_dir();
    };

    let nfo_path = cdir + &fd.fire_dir_nfos.to_string();
    let nfos_exist = Path::new(&nfo_path).is_dir();
    if !nfos_exist {
        fd.set_fire_dir_nfos();
    } else {
        fd.clean_nfos_dir();
    };
}

