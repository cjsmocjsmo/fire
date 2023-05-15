use byte_unit::Byte;
// use filesize::PathExt;
// use flate2::write::GzEncoder;
// use flate2::Compression;
// use glob::glob;
// use std::fs;
// use std::fs::File;
// use std::path::Path;
use walkdir::WalkDir;

pub fn media_total_size(addr: String) -> String {
    let total_size = WalkDir::new(addr)
        .min_depth(1)
        .max_depth(5)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    let btos = total_size.to_string();
    let result = Byte::from_str(btos).unwrap();
    let size = result.get_appropriate_unit(true).to_string();

    size
}

pub fn create_art_alb_list(alist: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut art_vec = Vec::new();
    let mut alb_vec = Vec::new();

    for a in alist {
        let tags = crate::setup::fire_mp3_info::get_tag_info(&a);
        let artist = tags.0;
        let album = tags.1;
        art_vec.push(artist);
        alb_vec.push(album)
    };

    // let mut art_list = art_vec.sort();
    // let mut alb_list = alb_vec.sort();
    art_vec.sort();
    alb_vec.sort();

    art_vec.dedup();
    alb_vec.dedup();

    (art_vec, alb_vec)
}

#[derive(Debug)]
pub struct Art_Id {
    id: String,
    artist: String,
    artistid: String,
}

pub fn create_artistids(alist: Vec<String>) -> Vec<Art_Id> {

    let mut art_id_list = Vec::new();
    let mut count = 0;
    for a in alist {
        count = count + 1;
    
        let af = crate::setup::fire_utils::FireUtils {
            apath: a.to_string()
        };
        
        let artistid = crate::setup::fire_utils::FireUtils::get_md5(&af);
        let artidstruc = Art_Id {
            id: count.clone().to_string(),
            artist: a.clone(),
            artistid: artistid.clone()
        };
        
        
        art_id_list.push(artidstruc);
        
    };


    art_id_list
}

#[derive(Debug)]
pub struct Alb_Id {
    id: String,
    album: String,
    albumid: String
}

pub fn create_albumids(alist: Vec<String>) -> Vec<Alb_Id> {
    let mut alb_id_list = Vec::new();
    let mut count = 0;
    for a in alist {
        count = count + 1;
    
        let af = crate::setup::fire_utils::FireUtils {
            apath: a.to_string()
        };
        
        let albumid = crate::setup::fire_utils::FireUtils::get_md5(&af);
        let albidstruc = Alb_Id {
            id: count.clone().to_string(),
            album: a.clone(),
            albumid: albumid.clone()
        };
        
        
        alb_id_list.push(albidstruc);
        
    };


    alb_id_list
}
// pub fn write_music_gzip_file() -> Result<(), std::io::Error> {
//     let music_meta = env::var("MTV_MUSIC_METADATA_PATH").unwrap();
//     let static_path = env::var("MTV_GZIP_PATH").unwrap();
//     let new_music_backup_path = static_path.clone() + "/MTV_Music_Meta_Backup.tar.gz";

//     let tar_gz = File::create(new_music_backup_path.clone())?;
//     let enc = GzEncoder::new(tar_gz, Compression::default());
//     let mut tar = tar::Builder::new(enc);
//     tar.append_dir_all("Music_Backups/", music_meta)?;
//     Ok(())
// }

// pub fn write_movie_gzip_file() -> Result<(), std::io::Error> {
//     let movie_meta = env::var("MTV_MOVIES_METADATA_PATH").unwrap();
//     let static_path = env::var("MTV_GZIP_PATH").unwrap();
//     let new_movie_backup_path = static_path.clone() + "/MTV_Movie_Meta_Backup.tar.gz";

//     let tar_gz = File::create(new_movie_backup_path.clone())?;
//     let enc = GzEncoder::new(tar_gz, Compression::default());
//     let mut tar = tar::Builder::new(enc);
//     tar.append_dir_all("Movie_Backups/", movie_meta)?;
//     Ok(())
// }

// pub fn write_tvshows_gzip_file() -> Result<(), std::io::Error> {
//     let tvshows_meta = env::var("MTV_TVSHOWS_METADATA_PATH").unwrap();
//     let static_path = env::var("MTV_GZIP_PATH").unwrap();
//     let new_tvshow_backup_path = static_path.clone() + "/MTV_TVShows_Meta_Backup.tar.gz";

//     let tar_gz = File::create(new_tvshow_backup_path.clone())?;
//     let enc = GzEncoder::new(tar_gz, Compression::default());
//     let mut tar = tar::Builder::new(enc);
//     tar.append_dir_all("TVShows_Backups/", tvshows_meta)?;
//     Ok(())
// }

// pub fn copy_gzip_files() -> u32 {
//     let gzip_path = env::var("MTV_GZIP_PATH").unwrap();
//     let static_path = env::var("MTV_STATIC_PATH").unwrap();
//     let glob_str = gzip_path + "/*.tar.gz";
//     let mut count = 0;
//     for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
//         count = count + 1;
//         let boo = e.unwrap().clone();
//         let fname = boo.file_name().unwrap().to_str().unwrap();
//         let new_path = static_path.clone() + "/" + fname;
//         fs::copy(boo, new_path).expect("File copy has failed");
//     }
//     count
// }
