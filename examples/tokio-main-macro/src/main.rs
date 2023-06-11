macro_rules! async_main {
    (async $body:block) => {
        fn main() {
            let body = async { $body };
            return tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(body);
        }
    };
}

use tokio::time::{sleep, Duration};

async_main!(async {
    sleep(Duration::from_millis(500)).await;
    println!("Hello world!");
});
