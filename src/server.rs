extern crate libfonthelper;
extern crate log;
extern crate simple_server;

use super::config::Config;
use super::routes::*;
use log::info;
use simple_server::{Method, Server};

pub fn init(config: Config) {
  let server = Server::new(|request, response| {
    info!("Request received. {} {}", request.method(), request.uri());

    match (request.method(), request.uri().path()) {
      (&Method::GET, "/figma/font-files") => get_files::handler(request, response),
      (&Method::GET, "/figma/font-file") => get_file::handler(request, response),
      (_, _) => any::handler(request, response),
    }
  });

  server.listen(&config.host, &config.port);
}
