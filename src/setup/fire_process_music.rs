// use json::object;
// use std::env;

#[derive(Debug, Default)]
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

    let id = crate::setup::fire_misc::get_md5(&x);
    let voodoo: &String = &"None".to_string();
    
    
    
    let duration_results = crate::setup::fire_mp3_info::get_duration(&x);

    let fullpath = &x.to_string();

    
    let base_dir = crate::setup::fire_split::split_base_dir(&x);

    let foo3 = crate::setup::fire_split::StrSplit {
        apath: x.clone()
    };

    let filename_results = crate::setup::fire_split::StrSplit::split_filename(&foo3);
    let music_artist_results = crate::setup::fire_split::StrSplit::music_split_artist(&foo3);

    let music_album_results = crate::setup::fire_split::StrSplit::music_split_album(&foo3);


    let ext = crate::setup::fire_split::StrSplit::split_ext(&foo3);
    let idx = index.to_string();
    let fsize_results = crate::setup::fire_misc::get_file_size(&x).to_string();

    // crate::setup::fire_x_info::write_music_json_to_file(
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
    println!("{:?}", music_info);
    music_info
    // println!("There are {} music files processed", &index);
}
