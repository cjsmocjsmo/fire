use glob::glob;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::YamlLoader;

use yaml_rust::Yaml;

fn get_current_working_dir() -> String {
    let path = env::current_dir().unwrap();
    let dir_path = String::from(path.to_string_lossy());

    dir_path
}

pub fn read_config() -> Vec<Yaml> {
    let mut file = File::open("./src/config.yaml").expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    docs
}

fn set_env_var(p1: String, p2: String) -> Result<(), Box<dyn std::error::Error>> {
    env::set_var(&p1, p2);
    let value = env::var(&p1);
    if value.is_err() {
        println!("Error: key not found");
    } else {
        println!("key is set to: {}", value.unwrap());
    }

    Ok(())
}







pub struct EnvVars {
    fire_docker_var: String,
    fire_scan_home_dir: String,
    fire_mongodb_address: String,
    fire_additional_media_path: String,
    fire_pagination: String,

}

impl EnvVars {
    fn set_docker_var(&self) -> bool {
        let dvar1 = String::from("FIRE_DOCKER_VAR");
        let dvar2 = self.fire_docker_var.clone();
        set_env_var(dvar1, dvar2).unwrap();
        true
    }
    fn set_scan_home_dir(&self) -> bool {
        let h1 = "FIRE_SCAN_HOME_DIR".to_string();
        let h2 = self.fire_scan_home_dir.clone();
        set_env_var(h1, h2).unwrap();
        true
    }
    fn set_fire_mongodb_address(&self) -> bool {
        let static1 = String::from("FIRE_MONGODB_ADDRESS");
        let static2 = self.fire_mongodb_address.clone();
        set_env_var(static1, static2).unwrap();
        true
    }
    fn set_fire_additional_media_path(&self) -> bool {
        let music0 = "FIRE_ADDITIONAL_MEDIA_PATH".to_string();
        let music1 = self.fire_additional_media_path.clone();
        set_env_var(music0, music1).unwrap();
        true
    }
    fn set_fire_pagination(&self) -> bool {
        let offset1 = String::from("FIRE_PAGINATION");
        let offset2 = self.fire_pagination.clone();
        set_env_var(offset1, offset2).unwrap();
        true
    }
    fn set_fire_dir_nfos(&self) -> bool {
        let nfo_dir = get_current_working_dir().to_string() + "/fire_dir/nfos";
        let nfos_dir_exists = Path::new(&nfo_dir).is_dir();
        if nfos_dir_exists {
            let nd = String::from("FIRE_NFOS");
            set_env_var(nd, nfo_dir.clone()).unwrap();
            clean_nfos_dir();
        } else {
            fs::create_dir_all(nfo_dir.clone()).unwrap();
            let nd = String::from("FIRE_NFOS");
            set_env_var(nd, nfo_dir.clone()).unwrap();
        }

        true
    }
    fn set_fire_dir_thumbnails(&self) -> bool {
        let thumb_dir = get_current_working_dir().to_string() + "/fire_dir/thumbnails";
        let thumb_dir_exists = Path::new(&thumb_dir).is_dir();
        if thumb_dir_exists {
            let td = String::from("FIRE_THUMBNAIL");
            set_env_var(td, thumb_dir.clone()).unwrap();
            clean_thumbnail_dir();
        } else {
            fs::create_dir_all(thumb_dir.clone()).unwrap();
            let td = String::from("FIRE_THUMBNAIL");
            set_env_var(td, thumb_dir.clone()).unwrap();
        }

        true
    }
    fn set_fire_dir_env(&self) -> bool {
        let fire_dir = get_current_working_dir().to_string() + "/fire_dir";
        let fire_dir_exists = Path::new(&fire_dir).is_dir();
        if fire_dir_exists {
            let f2 = String::from("FIRE_DIR");
            set_env_var(f2, fire_dir).unwrap();
        } else {
            fs::create_dir_all(fire_dir.clone()).unwrap();
            let f2 = String::from("FIRE_DIR");
            set_env_var(f2, fire_dir).unwrap();
        }

        true
    }
}

pub fn set_all_env_vars(paras: Vec<Yaml>) {
    for d in paras {
        let envvars = EnvVars {
            fire_docker_var: d["FIRE_DOCKER_VAR"].as_str().unwrap().to_string(),
            fire_scan_home_dir: d["FIRE_SCAN_HOME_DIR"].as_str().unwrap().to_string(),
            fire_mongodb_address: d["FIRE_MONGODB_ADDRESS"].as_str().unwrap().to_string(),
            fire_additional_media_path: d["FIRE_ADDITIONAL_MEDIA_PATH"].as_str().unwrap().to_string(),
            fire_pagination: d["FIRE_PAGINATION"].as_str().unwrap().to_string(),
        };

        if d["FIRE_DOCKER_VAR"].as_str().unwrap().to_string() == "nodocker" {
            envvars.set_fire_dir_env();
            envvars.set_fire_dir_thumbnails();
            envvars.set_fire_dir_nfos();
            envvars.set_docker_var();
            envvars.set_scan_home_dir();
            envvars.set_fire_mongodb_address();
            envvars.set_fire_additional_media_path();
            envvars.set_fire_pagination();
        };
    }
}

fn clean_nfos_dir() -> u32 {
    let movie_meta_dir_path = env::var("FIRE_NFOS").unwrap();
    let glob_str = movie_meta_dir_path + "/*.nfo";
    let mut count = 0;
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
        let rm_path = e.unwrap();
        count = count + 1;
        fs::remove_file(rm_path).expect("File delete failed");
    }

    count
}

fn clean_thumbnail_dir() -> u32 {
    let music_meta_dir_path = env::var("FIRE_THUMBNAIL").unwrap();
    let glob_str = music_meta_dir_path + "/*.jpg";
    let mut count = 0;
    for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
        count = count + 1;
        let rm_path = e.unwrap();
        fs::remove_file(rm_path).expect("File delete failed");
    }
    count
}
