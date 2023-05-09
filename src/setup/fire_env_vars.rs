use glob::glob;
use std::env;
use std::fs;
use std::path::Path;

fn get_current_working_dir() -> String {
    let path = env::current_dir().unwrap();
    let dir_path = String::from(path.to_string_lossy());

    dir_path
}

// pub fn read_config() -> Vec<Yaml> {
//     let mut file = File::open("./src/config.yaml").expect("Unable to open file");
//     let mut contents = String::new();

//     file.read_to_string(&mut contents)
//         .expect("Unable to read file");

//     let docs = YamlLoader::load_from_str(&contents).unwrap();

//     docs
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

pub struct EnvVars {
    fire_dir_nfos: String,
    fire_dir_thumbnails: String,
    fire_dir: String,

}

impl EnvVars {
    fn set_fire_dir_nfos(&self) -> bool {
        let nfo_dir = get_current_working_dir().to_string() + &self.fire_dir_nfos;
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
        let thumb_dir = get_current_working_dir().to_string() + &self.fire_dir_thumbnails;
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
    fn set_fire_dir(&self) -> bool {
        let fire_dir = get_current_working_dir().to_string() + &self.fire_dir;
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

pub fn set_all_env_vars() {
    let envvars = EnvVars {
        fire_dir_nfos: "/fire_dir/nfos".to_string(),
        fire_dir_thumbnails: "/fire_dir/thumbnails".to_string(),
        fire_dir: "/fire_dir".to_string(),
    };

    let cwd = get_current_working_dir().to_string();
    let cdir = cwd + "/fire_dir";

    let dir_exists = Path::new(&cdir).is_dir();

    // let docker_var = env::var("FIRE_DOCKER_VAR").expect("docker var not set");

    if dir_exists {
        clean_nfos_dir();
        clean_thumbnail_dir();
    } else {
        envvars.set_fire_dir();
        envvars.set_fire_dir_thumbnails();
        envvars.set_fire_dir_nfos();
    };
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
