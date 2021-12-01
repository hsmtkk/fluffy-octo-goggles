use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::redis::Commands;

fn main() {
    let redis_host = std::env::var("REDIS_HOST").expect("REDIS_HOST env var must be defined");
    let redis_url = format!("redis://{}", redis_host);
    let manager = RedisConnectionManager::new(redis_url).expect("failed to init manager");
    let pool = Pool::builder().build(manager).expect("failed to init connection pool");
    
    let mut conn = pool.get().expect("failed to get connection");

    // 1st read
    match conn.get::<_, String>("hoge") {
        Ok(val) => { println!("hoge = {}", val); },
        Err(_) => { println!("not found"); },
    }

    // create
    let _ : () = conn.set("hoge", "fuga").expect("failed to set value");

    // 2nd read
    match conn.get::<_, String>("hoge") {
        Ok(val) => { println!("hoge = {}", val); },
        Err(_) => { println!("not found"); },
    }
}
