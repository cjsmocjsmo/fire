// use json::object;
// use std::env;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::insert_into;

mod schema {
    diesel::table! {
        tvshowz {
            id -> Integer,
        fireid -> Text,
        index -> Text,
        category -> Text,
        name -> Text,
        season -> Text,
        episode -> Text,
        size -> Text,
        httppath -> Text,
        }
    }
}

// use schema::tvshowz;

#[derive(Queryable, PartialEq, Debug)]
struct TVShow {
    id: i32,
    fireid: String,
    index: String,
    category: String,
    name: String,
    season: String,
    episode: String,
    size: String,
    httppath: String,
}

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
        let idx = &n_split_vec.len() - 2;
        let parts: Vec<char> = n_split_vec[idx.clone()].chars().collect();

        let season = parts[1].to_string() + &parts[2].to_string();
        let episode = parts[4].to_string() + &parts[5].to_string();

        let results = (season, episode);

        results
    }

}



pub fn process_tvshows(tvshows_vec: Vec<String>) -> String{
    

    let mut count = 0;
    for tv in tvshows_vec {
        if tv.contains("TVShows") {
            count = count + 1;

            let tvshows = crate::setup::fire_utils::FireUtils {
                apath: tv.clone()
            };
            let tvshows2 = crate::setup::fire_process_tvshows::TVShowsUtils {
                apath: tv.clone()
            };

            let file_size = crate::setup::fire_utils::FireUtils::get_file_size(&tvshows);
            let catagory = crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_catagory(&tvshows2);
            let es = crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_episode_season(&tvshows2);
            
            

            let fire_id = crate::setup::fire_utils::FireUtils::get_md5(&tvshows);

            let fname = crate::setup::fire_utils::FireUtils::split_filename(&tvshows);
            let mut fnsplit_vec = Vec::new();
            let fnsplit = fname.split(" ");
            for f in fnsplit {
                fnsplit_vec.push(f.clone());
            };
            let episodename = fnsplit_vec.pop().unwrap();



            // let tvshows_obj = object! {
                
            //     fireid: fire_id,
            //     index: count.clone().to_string(),
            //     catagory: catagory.clone(),
            //     name: episodename.clone(),
            //     season: es.0,
            //     episode: es.1,
            //     size: file_size,
            //     httppath: tv
            // };

            // let tvshows_info = json::stringify(tvshows_obj.dump());

            // println!("{:?}", tvshows_info);

            // let fire_nfo_path =
            //     env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");

            // let a = format!("{}/", fire_nfo_path.as_str());
            // let b = format!("TVShows_{}_Meta.json", count.to_string());
            // let outpath = a + &b;

            // std::fs::write(outpath, tvshows_info).unwrap();

            // let tvs = TVShow {
            //     id: count.clone(),
            //     fireid: fire_id,
            //     index: count.clone().to_string(),
            //     category: catagory.clone(),
            //     name: episodename.to_string(),
            //     season: es.0,
            //     episode: es.1,
            //     size: file_size.clone(),
            //     httppath: tv.clone(),
            // };

            
            let mut conn = establish_connection();
            
            use schema::tvshowz::dsl::*;

            let values = &vec! [
                (
                    id.eq(count.clone()),
                    fireid.eq(fire_id),
                    index.eq(count.clone().to_string()),
                    category.eq(catagory.clone()),
                    name.eq(episodename.to_string()),
                    season.eq(es.0),
                    episode.eq(es.1),
                    size.eq(file_size.clone()),
                    httppath.eq(tv.clone()),
                )
            ];

            insert_into(tvshowz).values(values)
                .execute(&mut conn);
            
        }
    }
    count.to_string()
}


fn establish_connection() -> SqliteConnection {
    let url = ::std::env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&url).unwrap()
}





// fn insert_tvshows(conn: &mut SqliteConnection, fireid: String, index: String, category: String, name: String, season: String, episode: String, size: String, httppath: String) -> QueryResult<()> {
//     use diesel::dsl::insert_into;
//     use schema::tvshowz::dsl::*;

//     let tvs = TVShow {
//         id: index,
//         fireid: fireid,
//         index: index,
//         category: category,
//         name: name,
//         season: season,
//         episode: episode,
//         size: size,
//         httppath: httppath,
//     };

//     insert_into(tvshowz)
//         .values(&tvs)
//         .execute(conn)
// }