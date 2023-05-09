use json::object;
// use std::env;

pub struct TVShowsUtils {
    apath: String
}

impl TVShowsUtils {


    fn get_tv_catagory(&self) -> String {

        let foo12 = crate::setup::fire_utils::FireUtils {
            apath: self.apath.to_string()
        };

        let name = crate::setup::fire_utils::FireUtils::split_movie_name(&foo12);
        let n_split = name.split(" ");
        let mut n_split_vec = vec![];

        for n in n_split {
            n_split_vec.push(n);
        }

        let idx = n_split_vec.len() - 2;

        let mut newname_vec = vec![];

        let foo = n_split_vec.drain(0..idx);

        for f in foo {
            newname_vec.push(f);
        }

        let bar = newname_vec.join(" ");

        bar.to_string()
    }



    fn get_tv_episode_season(&self) -> (String, String) {
        let foo1 = crate::setup::fire_utils::FireUtils {
            apath: self.apath.to_string()
        };

        let name = crate::setup::fire_utils::FireUtils::split_movie_name(&foo1);
        let n_split = name.split(" ");
        let mut n_split_vec = vec![];

        for n in n_split {
            n_split_vec.push(n);
        }
        let idx = n_split_vec.len() - 2;

        let parts: Vec<char> = n_split_vec[idx].chars().collect();
        println!("{}", &self.apath);
        println!("{}", parts.len());
        println!("{:?}", parts);

        let season = parts[1].to_string() + &parts[2].to_string();
        let episode = parts[4].to_string() + &parts[5].to_string();

        let results = (season, episode);

        results
    }

}

// fn get_tv_poster_addr(x: &String) {

// }

pub fn process_tvshows(tvshows_vec: Vec<String>) -> String{
    // let tvshows_vec = crate::mtv_walk_dirs::walk_tvshows_dir();

    let mut count = 0;
    for tv in tvshows_vec {
        if tv.contains("TVShows") {
            println!("{}", tv);
            count = count + 1;

            let tvshows = crate::setup::fire_utils::FireUtils {
                apath: tv.clone()
            };
            let tvshows2 = crate::setup::fire_process_tvshows::TVShowsUtils {
                apath: tv.clone()
            };

            let file_size = crate::setup::fire_utils::FireUtils::get_file_size(&tvshows);
            let catagory = crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_catagory(&tvshows2);
            println!("{}", file_size);
            println!("{}", catagory);
            let es = crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_episode_season(&tvshows2);
            let season = es.0;
            let episode = es.1;

            let fname = crate::setup::fire_utils::FireUtils::split_filename(&tvshows);

            let tvshows_obj = object! {
                size: file_size,
                catagory: catagory,
                name: fname,
                season: season,
                episode: episode,
                path: tv
            };

            let tvshows_info = json::stringify(tvshows_obj.dump());

            println!("{:?}", tvshows_info);

            // let mtv_tvshows_metadata_path =
            //     env::var("MTV_TVSHOWS_METADATA_PATH").expect("$MTV_TVSHOWS_METADATA_PATH is not set");

            // let a = format!("{}/", mtv_tvshows_metadata_path.as_str());
            // let b = format!("TVShows_{}_Meta.json", count.to_string());
            // let outpath = a + &b;

            // std::fs::write(outpath, tvsows_info).unwrap();
        }
    }
    count.to_string()
}