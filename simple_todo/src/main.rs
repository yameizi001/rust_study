mod config;

fn main() {
    // load .env file
    dotenv::dotenv().ok();
    let cfg = config::Config::from_env().expect("Config error");
    println!("{:#?}", cfg);
}
