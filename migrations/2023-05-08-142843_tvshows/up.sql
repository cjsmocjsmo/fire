-- Your SQL goes here

CREATE TABLE tvshows (
    id INTEGER PRIMARY KEY,
    fireid TEXT,
    index TEXT,
    name TEXT,
    season TEXT,
    episode TEXT,
    size TEXT,
    httpmoviepath TEXT,
)
-- size: file_size,
--                 catagory: catagory,
--                 name: fname,
--                 season: season,
--                 episode: episode,
--                 path: tv
