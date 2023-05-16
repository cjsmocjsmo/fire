use std::env;
use std::clone::Clone;
// use mongodb::Client;
// use mongodb::bson::to_document;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicInfo {
    id: String,
    imgurl: String,
    artist: String,
    album: String,
    song: String,
    basedir: String,
    filenameresults: String,
    musicartistresults: String,
    musicalbumresults: String,
    durationresults: String,
    fullpath: String,
    extension: String,
    index: String,
    page: String,
    fsizeresults: String,
}

pub fn process_mp3s(x: String, index: String, page: String) -> MusicInfo {
    let tags = crate::setup::fire_mp3_info::get_tag_info(&x);
    let artist = tags.0;
    let album = tags.1;
    let song = tags.2;
    let voodoo: &String = &"None".to_string();
    let fu = crate::setup::fire_utils::FireUtils {
        apath: x.clone()
    };
    let id = crate::setup::fire_utils::FireUtils::get_md5(&fu);
    let duration_results = crate::setup::fire_mp3_info::get_duration(&x);
    let fullpath = &x.to_string();
    let base_dir = crate::setup::fire_utils::FireUtils::split_base_dir(&fu);
    let filename_results = crate::setup::fire_utils::FireUtils::split_filename(&fu);
    let music_artist_results = crate::setup::fire_utils::FireUtils::music_split_artist(&fu);
    let music_album_results = crate::setup::fire_utils::FireUtils::music_split_album(&fu);
    let ext = crate::setup::fire_utils::FireUtils::split_ext(&fu);
    let idx = index.to_string();
    let fsize_results = crate::setup::fire_utils::FireUtils::get_file_size(&fu).to_string();

    let music_info = MusicInfo { 
        id: id,
        imgurl: voodoo.to_string(),
        artist: artist,
        album: album,
        song: song,
        basedir: base_dir,
        filenameresults: filename_results,
        musicartistresults: music_artist_results,
        musicalbumresults: music_album_results,
        durationresults: duration_results,
        fullpath: fullpath.to_string(),
        extension: format!("{:?}", ext),
        index: idx,
        page: page.to_string(),
        fsizeresults: fsize_results,
    };
    write_music_nfos_to_file(music_info.clone(), index.clone());
    // let database = client.database("fire");
    //         let collection = database.collection("music_main");
    //         let bson_document = to_document(&music_info)?;
    //         collection.insert_one(bson_document, None).await?;
    println!("{:#?}", music_info.clone());

    music_info.clone()
}

fn write_music_nfos_to_file(mfo: MusicInfo, index: String) {
    let mus_info = serde_json::to_string(&mfo).unwrap();
    let fire_music_metadata_path =
        env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");
    let a = format!("{}/", fire_music_metadata_path.as_str());
    let b = format!("Music_Meta_{}.json", index.to_string());
    let outpath = a + &b;
    std::fs::write(outpath, mus_info).unwrap();
}
