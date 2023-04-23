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

// pub fn get_docker_var() -> String {
//     let docker_var_results = env::var("FIRE_DOCKER_VAR");
//     let docker_var = match docker_var_results {
//         Ok(docker_var) => docker_var,
//         Err(_error) => "docker var not set".to_string(),
//     };

//     docker_var
// }

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

pub fn read_config() -> Vec<Yaml> {
    let mut file = File::open("./src/config.yaml").expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    docs
}

fn set_fire_dir_env(cwd: &String) {
    let fire_dir = cwd.to_string() + "/fire_dir";
    let fire_dir_exists = Path::new(&fire_dir).is_dir();
    if fire_dir_exists {
        let f2 = String::from("FIRE_DIR");
        set_env_var(f2, fire_dir).unwrap();
    } else {
        fs::create_dir_all(fire_dir.clone()).unwrap();
        let f2 = String::from("FIRE_DIR");
        set_env_var(f2, fire_dir).unwrap();
    }
}

fn set_fire_dir_thumbnails(cwd: &String) {
    let thumb_dir = cwd.to_string() + "/fire_dir/thumbnails";
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
}

fn set_fire_dir_nfos(cwd: &String) {
    let nfo_dir = cwd.to_string() + "/fire_dir/nfos";
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
}

fn set_fire_docker_var(dvar: String) {
    let dvar1 = String::from("FIRE_DOCKER_VAR");
    let dvar2 = dvar;
    set_env_var(dvar1, dvar2).unwrap();
}

fn set_fire_mongodb_address(addr: String) {
    let static1 = String::from("FIRE_MONGODB_ADDRESS");
    let static2 = addr;
    set_env_var(static1, static2).unwrap();
}

fn set_fire_pagination() {
    let offset1 = String::from("FIRE_PAGINATION");
    let offset2 = String::from("25");
    set_env_var(offset1, offset2).unwrap();
}

fn set_fire_additional_media_path(med_path: String) {
    let music0 = "FIRE_ADDITIONAL_MEDIA_PATH".to_string();
    let music1 = med_path;
    set_env_var(music0, music1).unwrap();
}

pub fn set_all_env_vars(paras: Vec<Yaml>) {
    for d in paras {
        if d["FIRE_DOCKER_VAR"].as_str().unwrap().to_string() == "nodocker" {
            // set docker_var for future runs

            let cwd = get_current_working_dir();

            set_fire_dir_env(&cwd);
            set_fire_dir_thumbnails(&cwd);
            set_fire_dir_nfos(&cwd);
            set_fire_docker_var(d["FIRE_DOCKER_VAR"].as_str().unwrap().to_string());
            set_fire_additional_media_path(d["FIRE_ADDITIONAL_MEDIA_PATH"].as_str().unwrap().to_string());

            set_fire_mongodb_address(d["FIRE_MONGODB_ADDRESS"].as_str().unwrap().to_string());
            set_fire_pagination();
        };

        // let addr1 = String::from("FIRE_SERVER_ADDRESS");
        // let addr2 = String::from("http://192.168.0.94");
        // set_env_var(addr1, addr2).unwrap();

        // let p1 = String::from("FIRE_SERVER_PORT");
        // let p2 = String::from("8888");
        // set_env_var(p1, p2).unwrap();

        // let gzip1 = String::from("FIRE_GZIP_PATH");
        // let gzip2 = d["FIRE_GZIP_PATH"].as_str().unwrap().to_string();
        // set_env_var(gzip1, gzip2).unwrap();

        // let music2 = "FIRE_MUSIC_PATH".to_string();
        // let music3 = d["FIRE_MUSIC_PATH"].as_str().unwrap().to_string();
        // set_env_var(music2, music3).unwrap();

        // let music4 = "FIRE_MUSIC_THUMBNAIL_PATH".to_string();
        // let music5 = d["FIRE_MUSIC_THUMBNAIL_PATH"].as_str().unwrap().to_string();
        // set_env_var(music4, music5).unwrap();

        // let music6 = "FIRE_MUSIC_METADATA_PATH".to_string();
        // let music7 = d["FIRE_MUSIC_METADATA_PATH"].as_str().unwrap().to_string();
        // set_env_var(music6, music7).unwrap();

        // let music8 = "FIRE_MOVIES_PATH".to_string();
        // let music9 = d["FIRE_MOVIES_PATH"].as_str().unwrap().to_string();
        // set_env_var(music8, music9).unwrap();

        // let music10 = "FIRE_MOVIES_THUMBNAIL_PATH".to_string();
        // let music11 = d["FIRE_MOVIES_THUMBNAIL_PATH"].as_str().unwrap().to_string();
        // set_env_var(music10, music11).unwrap();

        // let music12 = "FIRE_MOVIES_METADATA_PATH".to_string();
        // let music13 = d["FIRE_MOVIES_METADATA_PATH"].as_str().unwrap().to_string();
        // set_env_var(music12, music13).unwrap();

        // let music14 = "FIRE_TVSHOWS_PATH".to_string();
        // let music15 = d["FIRE_TVSHOWS_PATH"].as_str().unwrap().to_string();
        // set_env_var(music14, music15).unwrap();

        // let music16 = "FIRE_TVSHOWS_METADATA_PATH".to_string();
        // let music17 = d["FIRE_TVSHOWS_METADATA_PATH"].as_str().unwrap().to_string();
        // set_env_var(music16, music17).unwrap();

        // let music18 = "FIRE_TVSHOWS_POSTERS_PATH".to_string();
        // let music19 = d["FIRE_TVSHOWS_POSTERS_PATH"].as_str().unwrap().to_string();
        // set_env_var(music18, music19).unwrap();

        // let music18 = "FIRE_TVSHOWS_THUMBNAIL_PATH".to_string();
        // let music19 = d["FIRE_TVSHOWS_THUMBNAIL_PATH"]
        //     .as_str()
        //     .unwrap()
        //     .to_string();
        // set_env_var(music18, music19).unwrap();

        // let static1 = String::from("FIRE_MOVIES_POSTERS_PATH");
        // let static2 = d["FIRE_MOVIES_POSTERS_PATH"].as_str().unwrap().to_string();
        // set_env_var(static1, static2).unwrap();

        // let static1 = String::from("FIRE_STATIC_PATH");
        // let static2 = d["FIRE_STATIC_PATH"].as_str().unwrap().to_string();
        // set_env_var(static1, static2).unwrap();
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
