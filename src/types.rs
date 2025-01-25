use actix_web::Route;
pub type CallBack = Box<dyn Fn() -> Route + Send + Sync>;
pub type ProvideUrl = Vec<(String, CallBack)>;
pub type Scope = String;
pub type Provides = Vec<String>;
