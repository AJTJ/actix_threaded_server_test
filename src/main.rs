use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use std::{sync::mpsc, thread::sleep};
use std::{thread, time::Duration};

#[actix_web::main]
async fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut sys = System::new("http-server");
        let srv = HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
            .bind("127.0.0.1:8080")?
            .shutdown_timeout(60)
            .run();

        let _ = tx.send(srv.clone());
        sys.block_on(srv)
    });

    let srv = rx.recv().unwrap();
    // pause accepting new connections
    srv.pause().await;
    println!("svr paused sleeping for 5");
    sleep(Duration::from_secs(5));
    // resume accepting new connections
    srv.resume().await;
    println!("svr resumed sleeping for 5");
    sleep(Duration::from_secs(5));
    // stop server
    srv.stop(true).await;
}
