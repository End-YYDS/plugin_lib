use actix_web::Route;
pub type CallBack = Box<dyn Fn() -> Route + Send + Sync>;
pub type ProvideUrl = Vec<(String, CallBack)>;
pub type Scope = String;
pub type Provides = Vec<String>;

#[derive(Debug, serde::Serialize)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub scope: Scope,
    pub routes: Provides,
    pub signature: Option<String>,
    pub frontend: Option<String>,
}
