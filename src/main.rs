use warp::Filter;
use serde_json;


use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::DirBuilder;


fn create_new_page(body: HashMap<String,String>, config_dir: &PathBuf) -> String {
    let title = body.get("filename").expect("missing `filename` in json body.").to_string();
    
    let mut hasher = DefaultHasher::new();
    title.hash(&mut hasher);
    let hash = hasher.finish();

    // save the source json to /page_source/ with an id (hashed value of the page filename (TODO:
    // update to use the filename that's passed in the json. That way we don't need to do any yaml
    // parsing on the clientside, just provide a json body that consists of the filename, the
    // format, and the  page content. Any parsing and templating of the final page will be the 
    // responsibility of the final script that does the conversion.)

    let page_source_file = config_dir.join(format!("page_source/{}", hash));
    std::fs::write(page_source_file.clone(), serde_json::to_string(&body).unwrap());

    let content = body.get("content").unwrap().to_string();
    let html_content = convert_body(content, body.get("source_markup_format").unwrap().to_string());

    let page_file = config_dir.join(format!("page/{}", hash));
    std::fs::write(page_file.clone(), html_content);

    format!("saved page with hash: {}", hash)
}

/// convert `content` into HTML. At the moment doesn't do anything, just returns the page raw.
fn convert_body(content: String, _source_markup_format: String) -> String {
    // checks the config for the appropriate script to feed the content string into.
    // config is in xdg_dirs.place_config_file("config.toml");
    //      looks like:
    //          [
    //              { "format": "markdown", "script": "/home/reed/bin/pandoc-blog-convert.sh" },
    //              { "format": "fenneldown", "script": "/home/reed/bin/fenneldown-blog-convert.sh" },
    //          ]
    // echo the content into the script as stdin, save the corresponding stdout/stderr (if stderr
    // has anything in it, we should report that to the user somehow in the HTTP response.)
    // return the output.
    content
}

fn init_data_dirs(data_dir: &PathBuf) {
    DirBuilder::new()
        .recursive(true)
        .create(data_dir.join("page_source")).unwrap();
    DirBuilder::new()
        .recursive(true)
        .create(data_dir.join("page")).unwrap();
}


#[tokio::main]
async fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("dg")
        .unwrap();


    // setting up pages
    init_data_dirs(&xdg_dirs.get_data_home());

    let new_page = warp::post()
        .and(warp::path("new_page"))
        .and(warp::body::json())
        .map(move |x| create_new_page(x, &xdg_dirs.get_data_home()));

    warp::serve(new_page)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
