pub mod types;
use std::fmt::Debug;

use types::{ProvideUrl, Scope};
pub trait Plugin: Send + Sync + Debug {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn scope(&self) -> Scope;
    fn register_routes(&self) -> ProvideUrl;
    fn signature(&self) -> Option<String> {
        None
    }
    fn frontend_file(&self) -> Option<String> {
        None
    }
}
impl Plugin for Box<dyn Plugin> {
    fn name(&self) -> &str {
        self.as_ref().name()
    }
    fn version(&self) -> &str {
        self.as_ref().version()
    }
    fn description(&self) -> &str {
        self.as_ref().description()
    }
    fn scope(&self) -> Scope {
        self.as_ref().scope()
    }
    fn register_routes(&self) -> ProvideUrl {
        self.as_ref().register_routes()
    }
    fn signature(&self) -> Option<String> {
        self.as_ref().signature()
    }
    fn frontend_file(&self) -> Option<String> {
        self.as_ref().frontend_file()
    }
}

/// 定義一個新的插件
///
/// # Arguments
///
/// * `plugin_struct` - 實作 Plugin trait 的結構體
/// * `name` - 插件名稱
/// * `version` - 插件版本號
/// * `description` - 插件描述
/// * `scope` - 插件作用域
/// * `functions` - 路由設定
///    * `path` - 路由路徑
///    * `method` - HTTP方法
///    * `handler` - 處理函數
///
/// # Example
/// ```rust
/// struct MyPlugin;
/// impl MyPlugin {
///    pub fn new() -> Self {
///       Self
///   }
///   async fn hello_handler() -> impl actix_web::Responder {
///       "Hello, world!"
///  }
/// }
///
/// declare_plugin!(
///     MyPlugin,
///     "example-plugin",
///     "1.0.0",
///     "A demo plugin",
///     "global",
///     functions: {
///         "/api/hello" => {method: actix_web::web::get(), handler: MyPlugin::hello_handler}
///     }
/// );
/// ```
#[macro_export]
macro_rules! declare_plugin {
    (
        $plugin_struct: ident,
        meta: {
            $name:expr,
            $version:expr,
            $description:expr,
            $scope:expr,
            $sig:expr
        },
        $frontend_file:expr,
        functions: {$($path:expr => {method: $method:expr, handler: $handler:expr}),* $(,)?}
    ) => {
        impl plugin_lib::Plugin for $plugin_struct {
            fn name(&self) -> &str {
                $name
            }
            fn version(&self) -> &str {
                $version
            }
            fn description(&self) -> &str {
                $description
            }
            fn scope(&self) -> String {
                $scope.to_string()
            }
            fn register_routes(&self) -> Vec<(String, Box<dyn Fn() -> actix_web::Route + Send + Sync>)> {
                vec![
                    $(
                        ($path.to_string(),Box::new(|| $method.to($handler))),
                    )*
                ]
            }
            fn frontend_file(&self) -> Option<String> {
                match $frontend_file {
                    "" => None,
                    file => Some(file.to_string())
                }
            }
            fn signature(&self) -> Option<String> {
                match $sig {
                    "" => None,
                    sig => Some(sig.to_string())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        #[allow(improper_ctypes_definitions)]
        pub unsafe extern "C" fn _create_plugin() -> *mut dyn plugin_lib::Plugin {
            Box::into_raw(Box::new(<$plugin_type>::new()))
        }
    };
}
