// use json::object;
use std::env;

#[derive(Debug)]
struct MusicInfo {
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

pub fn process_mp3s(mp3svec: Vec<String>) -> String {
    // let mp3svec = crate::setup::fire_walk_dirs::walk_music_dir_music();

    let mut index = 0;

    let mut page = 1;

    let mut page_count = 0;

    let ofs = env::var("FIRE_PAGINATION").unwrap();
    let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    // let mut not_named_correctly = vec![];

    for mp3 in mp3svec {
        index = index + 1;
        if page_count < offset {
            page_count = page_count + 1;
            page = page;
        } else {
            page_count = 1;
            page = page + 1;
        }

        let tags = crate::setup::fire_mp3_info::get_tag_info(&mp3);
        let artist = tags.0;
        let album = tags.1;
        let song = tags.2;

        let test_results = crate::setup::fire_nnc_info::test_media_sameness(
            mp3.clone(),
            artist.clone(),
            album.clone(),
            song.clone(),
        );
        if test_results {
            let id = crate::setup::fire_misc::get_md5(&mp3);
            let voodoo: &String = &"None".to_string();
            let base_dir = crate::setup::fire_split::split_base_dir(&mp3);
            let filename_results = crate::setup::fire_split::split_filename(&mp3);
            let music_artist_results = crate::setup::fire_split::music_split_artist(&base_dir);
            let music_album_results = crate::setup::fire_split::music_split_album(&base_dir);
            let duration_results = crate::setup::fire_mp3_info::get_duration(&mp3);

            let fullpath = &mp3.to_string();
            let ext = crate::setup::fire_split::split_ext(&mp3);
            let idx = index.to_string();
            let fsize_results = crate::setup::fire_misc::get_file_size(&mp3).to_string();

            // crate::setup::fire_mp3_info::write_music_json_to_file(
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
                extension: ext,
                index: idx,
                page: page.to_string(),
                fsizeresults: fsize_results,
            };
            println!("{:?}", music_info);
        };
        // else {
        //     println!("\n\tThe file parts and tags do not match:\n\t\t{}", &mp3);
        //     let not_named_correctly_entry = object! { filename:&*mp3 };
        //     not_named_correctly.push(not_named_correctly_entry);
        // };
    }

    // let not_named_correctly_count = not_named_correctly.len();

    // let niv = json::stringify(not_named_correctly.clone());

    // let fire_music_metadata_path =
    //     env::var("fire_MUSIC_METADATA_PATH").expect("$fire_MUSIC_METADATA_PATH is not set");

    // let a = format!("{}/", fire_music_metadata_path.as_str());
    // let b = format!("Named_Incorrectly.json");
    // let outpath = a + &b;

    // std::fs::write(outpath, niv).unwrap();

    // println!(
    //     "There are {} music files named incorrectly",
    //     not_named_correctly_count
    // );

    index.to_string()
    // println!("There are {} music files processed", &index);
}
