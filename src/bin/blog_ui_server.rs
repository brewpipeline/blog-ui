use std::error::Error;
use std::path::PathBuf;

use bytes::Bytes;
use clap::Parser;
use futures::stream::{self, Stream, StreamExt};
use blog_ui::*;
use warp::Filter;

static APP_TAG: &str = "<div id=\"app\">";

type BoxedError = Box<dyn Error + Send + Sync + 'static>;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[structopt(short, long)]
    dir: PathBuf,
}

fn render(
    index_html_before: String,
    index_html_after: String,
) -> Box<dyn Stream<Item = Result<Bytes, BoxedError>> + Send> {
    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: "/".into(),
        queries: Default::default(),
    });

    Box::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(|m| Result::<_, BoxedError>::Ok(m.into())),
    )
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once(APP_TAG).unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str(APP_TAG);
    let index_html_after = index_html_after.to_owned();

    let html = warp::path::end().then(move || {
        let index_html_before = index_html_before.clone();
        let index_html_after = index_html_after.clone();

        async move { warp::reply::html(render(index_html_before, index_html_after)) }
    });

    let routes = html.or(warp::fs::dir(opts.dir));

    println!("You can view the website at: http://localhost:8080/");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}