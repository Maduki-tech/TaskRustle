use axum::{Router, routing::get};


pub struct Server{
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port,
        }
    }

    
    #[tokio::main]
    pub async fn start(&self) {
        let app = Router::new().route("/", get(|| async {"Hello, World!"}));
        let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listner, app).await.unwrap();
        
    }
}


