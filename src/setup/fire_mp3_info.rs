use id3::{Tag, TagLike};
use json::{object, JsonValue};
use mp3_duration;
use std::env;
use std::path::Path;
use std::time::Duration;

// pub fn get_tag_info(x: &String) -> (String, String, String) {
//     let tag = Tag::read_from_path(x).unwrap();
//     let artist = tag.artist().unwrap().to_string();
//     let album = tag.album().unwrap().to_string();
//     let song = tag.title().unwrap().to_string();

//     (artist, album, song)
// }

pub fn get_tag_info(x: &String) -> (String, String, String) {
    let tag = Tag::read_from_path(x).unwrap();
    let artist = tag.artist().expect(x);
    let album = tag.album().expect(x);
    let song = tag.title().expect(x);

    (artist.to_string(), album.to_string(), song.to_string())
}

fn mp3_duration_extract(x: String) -> Duration {
    let path = Path::new(&x);
    let dur_sec_res = mp3_duration::from_path(&path);
    let dur_sec = match dur_sec_res {
        Ok(d) => d,
        Err(_) => Duration::new(0, 0),
    };

    dur_sec
}

pub fn get_duration(x: &String) -> String {
    let dur_sec = mp3_duration_extract(x.to_string());

    if dur_sec == Duration::new(0, 0) {
        let dur_min = dur_sec.div_f32(60.0);
        let dur_str = format!("{:?}", dur_min);
        let mut durvec = vec![];
        for i in dur_str.chars() {
            durvec.push(i);
        }

        let mut newvec = vec![];
        let mut count: i32 = 0;
        for c in durvec {
            count = count + 1;
            if count < 5 {
                newvec.push(c);
            } else {
                break;
            };
        }

        let duration: String = newvec.into_iter().collect();
        return duration;
    } else {
        let new_dur = Duration::new(0, 0);
        let duration = format!("{:?}", new_dur);
        return duration;
    };
}

pub fn write_music_json_to_file(
    id: String,
    voodoo: String,
    artist: String,
    album: String,
    song: String,
    base_dir: String,
    filename_results: String,
    music_artist_results: String,
    music_album_results: String,
    duration_results: String,
    fullpath: String,
    extension: String,
    idx: String,
    page: String,
    fsize_results: String,
) -> JsonValue {
    // if artc == true && albc == true && sc == true {

    let mp3_info = object! {
        mp3id: id,
        fullpath: fullpath,
        basedir: base_dir,
        filename: filename_results,
        ext: extension,
        imgurl: &*voodoo,
        mp3_url: &*voodoo,
        tag_artist: artist,
        tag_album: album,
        tag_title: song,
        idx: idx.clone(),
        page: page,
        fsize: fsize_results,
        filename_artist: &*music_artist_results,
        filename_album: &*music_album_results,
        duration: duration_results,
    };

    let mfo: String = json::stringify(mp3_info.dump());

    let mtv_music_metadata_path =
        env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

    let a = format!("{}/", mtv_music_metadata_path.as_str());
    let b = format!("Music_File_Meta_{}.json", &idx);
    let outpath = a + &b;
    std::fs::write(outpath, mfo.clone()).unwrap();

    return object! {filename: "Success".to_string() };
}
