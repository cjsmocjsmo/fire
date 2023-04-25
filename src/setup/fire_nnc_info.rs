

fn get_dir_artist2(x: String) -> String {
    let boo = x.split("/");
    let mut newboovec = vec![];
    for b in boo {
        newboovec.push(b);
    }
    newboovec.pop();
    newboovec.pop();
    let foobar = newboovec.pop().unwrap();
    let foo = foobar.split("_");
    let mut bar = vec![];
    for f in foo {
        bar.push(f);
    }
    let artist = bar.join(" ");

    artist
}

pub fn get_dir_album2(x: String) -> String {
    
    let boo = x.split("/");
    let mut newboovec = vec![];
    for b in boo {
        newboovec.push(b);
    }
    newboovec.pop();
    let foobar = newboovec.pop().unwrap();
    let foo = foobar.split("_");
    let mut bar = vec![];
    for f in foo {
        bar.push(f);
    }
    let album = bar.join(" ");

    album
}

fn get_fn_artist2(x: String) -> (String, String, String) {
    let art_split1 = x.split("_-_");
    let mut art_split1_vec = vec![];
    for art in art_split1 {
        art_split1_vec.push(art);
    }

    let artist_res = art_split1_vec[1];
    let art = artist_res.split("_");
    let mut art_vec = vec![];
    for a in art {
        art_vec.push(a);
    }
    let artist = art_vec.join(" ");

    let album_res = art_split1_vec[2];
    let alb = album_res.split("_");
    let mut alb_vec = vec![];
    for a in alb {
        alb_vec.push(a);
    }
    let album = alb_vec.join(" ");


    let song_res = art_split1_vec[3];
    let son = song_res.split("_");
    let mut son_vec = vec![];
    for a in son {
        son_vec.push(a);
    }
    let song_res2 = son_vec.join(" ");
    let son_spt =  song_res2.split(".");
    let mut new_song_vec = vec![];
    for s in son_spt {
        new_song_vec.push(s);
    };

    let song = new_song_vec[0];
    

    let meta = (artist.to_string(), album.to_string(), song.to_string());

    meta
}

pub fn test_media_sameness(
    fpath: String,
    tag_artist: String, 
    tag_album: String, 
    tag_song: String

) -> bool {
    let dir_album = get_dir_album2(fpath.clone());
    let dir_artist = get_dir_artist2(fpath.clone());
    let meta = get_fn_artist2(fpath.clone());
    let file_artist = meta.0;
    let file_album = meta.1;
    let file_song = meta.2;

    let mut result_vec = vec![];

    if dir_artist != file_artist || file_artist != tag_artist || dir_artist != tag_artist {
        let zoo = format!(
            "Artist Dont Match:\n\t dir: {}, file: {}, tag: {}",
            dir_artist, file_artist, tag_artist
        );
        println!("{}", zoo);
        result_vec.push(false);
    } else {
        result_vec.push(true);
    }

    if dir_album != file_album || file_album != tag_album || dir_album != tag_album {
        let moo = format!(
            "Albums Dont Match:\n\t dir: {}, file: {}, tag: {}",
            dir_album, file_album, tag_album
        );
        println!("{}", moo);
        result_vec.push(false);
    } else {
        result_vec.push(true);
    }

    if file_song != tag_song {
        let boo = format!(
            "Songs Dont Match:\n\t file: {}, tag: {}",
            file_song, tag_song
        );
        println!("{}", boo);
        result_vec.push(false);
    } else {
        result_vec.push(true);
    };

    let mut result = true;
    if result_vec.contains(&false) {
        result = false;
    };

    result
    
}