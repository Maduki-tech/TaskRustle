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
        let app = Router::new().route("/", get(self.get_handler()));
        let addr = "0.0.0.0:".to_string() + &self.port.to_string();
        println!("Listening on {}", addr);
        let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listner, app).await.unwrap();
    }

    fn get_handler(&self) -> String {
        "hello world".to_string()
    }


}


