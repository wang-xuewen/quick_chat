在 Rust 中，实现一个全局可访问的 `web_server` 对象，并用 `axum` 创建一个 Web 服务器和登录 API，可以通过使用 `lazy_static` 或 `once_cell` 来初始化全局静态变量。以下是一个完整的示例代码：

### 示例代码
```rust
use axum::{
    routing::{post, get},
    Router,
    Json,
    extract::{Extension, Path},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceBuilder;

// 定义全局 web_server 对象
static WEB_SERVER: Lazy<Arc<Mutex<WebServer>>> = Lazy::new(|| {
    Arc::new(Mutex::new(WebServer::new()))
});

// 定义 WebServer 结构体
pub struct WebServer;

impl WebServer {
    pub fn new() -> Self {
        WebServer
    }

    pub async fn start(&self) {
        // 构建路由
        let app = Router::new()
            .route("/", get(|| async { "Welcome to the Web Server!" }))
            .route("/login", post(login_handler))
            .layer(ServiceBuilder::new().layer(Extension(WEB_SERVER.clone())));

        // 启动 Axum Web 服务器
        let addr = "127.0.0.1:3000".parse().unwrap();
        println!("Listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

// 定义登录请求和响应结构
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
}

// 处理登录请求
async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    if payload.username == "admin" && payload.password == "password" {
        Json(LoginResponse {
            message: "Login successful!".to_string(),
        })
    } else {
        Json(LoginResponse {
            message: "Invalid username or password.".to_string(),
        })
    }
}

#[tokio::main]
async fn main() {
    // 访问全局 web_server 对象并启动服务
    let web_server = WEB_SERVER.clone();
    let server = web_server.lock().await;
    server.start().await;
}
```

### 代码说明
1. **全局对象 `WEB_SERVER`**：
   - 使用 `once_cell::sync::Lazy` 创建线程安全的全局对象。
   - 包装在 `Arc<Mutex<T>>` 中，确保全局的并发安全和可变性。

2. **`axum` 路由**：
   - 创建两个路由：根路径 `"/"` 返回欢迎信息，`"/login"` 是 POST 接口，用于模拟登录验证。
   - 使用 `Json` 提取请求和响应。

3. **登录逻辑**：
   - 对用户名和密码进行简单校验，返回相应的 JSON 消息。

4. **启动服务器**：
   - `start` 方法通过 Axum 启动 Web 服务器，监听 `127.0.0.1:3000`。

### 如何运行
1. 添加依赖到 `Cargo.toml`：
```toml
[dependencies]
axum = "0.6"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
once_cell = "1.17"
tower = "0.4"
```

2. 使用 `cargo run` 启动服务。

### 测试登录接口
可以使用 `curl` 或 Postman 测试：
```bash
curl -X POST -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"password"}' \
    http://127.0.0.1:3000/login
```

返回成功：
```json
{"message":"Login successful!"}
```

或者失败：
```json
{"message":"Invalid username or password."}
```


// ---------------------------------------------------------------------------------------


如果不希望在路由定义时直接绑定处理器，可以进一步解耦路由定义和处理逻辑，将路由定义抽象为一个统一的注册函数，类似 Beego 的做法。以下是实现分离的改进版：

---

### 示例代码

```rust
use axum::{
    routing::{get, post, MethodRouter},
    Router,
};
use std::collections::HashMap;

// 全局注册表类型
type HandlerFn = fn() -> &'static str;

// 创建一个路由注册器
pub struct RouteRegistry {
    routes: HashMap<&'static str, (axum::routing::MethodRouter, &'static str)>,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn register<F>(
        &mut self,
        path: &'static str,
        method: axum::routing::MethodRouter,
        handler: &'static str,
    ) {
        self.routes.insert(path, (method, handler));
    }

    pub fn build_app(&self) -> Router {
        self.routes.iter().fold(Router::new(), |app, (p, (method, action))| {
            app.route(*p, action.clone())
        })
    }

}


#[tokio::main]
async fn main() {

以下是分离逻辑后的完整示例代码：

---

### 主程序

```rust
use axum::{
    routing::{get, post, MethodRouter},
    Router,
};
use std::collections::HashMap;

// 全局路由注册器类型
type HandlerFn = fn() -> &'static str;

pub struct RouteRegistry {
    routes: HashMap<&'static str, MethodRouter>,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn register<F>(
        &mut self,
        path: &'static str,
        method: MethodRouter,
    ) {
        self.routes.insert(path, method);
    }

    pub fn build_app(&self) -> Router {
        let mut router = Router::new();
        for (path, method) in &self.routes {
            router = router.route(path, method.clone());
        }
        router
    }
}

// 路由处理器
async fn index_handler() -> &'static str {
    "Welcome to the Index Page!"
}

async fn captcha_handler() -> &'static str {
    "This is a Captcha endpoint."
}

async fn login_handler() -> &'static str {
    "This is a Login endpoint!"
}

async fn fallback_handler() -> &'static str {
    "404 Not Found"
}

#[tokio::main]
async fn main() {
    // 创建路由注册器
    let mut registry = RouteRegistry::new();

    // 注册路由
    registry.register("/index.html", get(index_handler));
    registry.register("/captcha", get(captcha_handler));
    registry.register("/api/login", post(login_handler));

    // 构建应用路由
    let app = registry.build_app().fallback(fallback_handler);

    // 启动服务
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

### 改进点

1. **路由定义分离**：
   - 路由与处理器分离，通过 `register` 方法定义路径和 HTTP 方法，无需在同一处绑定处理逻辑。

2. **集中化管理**：
   - 使用 `RouteRegistry` 集中存储所有的路由映射，可以统一管理路径。

3. **灵活拓展**：
   - 如果需要动态注册路由，直接调用 `register` 添加新路由即可。

---

### 测试结果

与之前类似，通过以下命令进行测试：

- **首页**: `curl http://127.0.0.1:3000/index.html`
  ```text
  Welcome to the Index Page!
  ```

- **验证码接口**: `curl http://127.0.0.1:3000/captcha`
  ```text
  This is a Captcha endpoint.
  ```

- **登录接口**: `curl -X POST http://127.0.0.1:3000/api/login`
  ```text
  This is a Login endpoint!
  ```

- **未定义路由**: `curl http://127.0.0.1:3000/unknown`
  ```text
  404 Not Found
  ```

---

### 优势

1. **代码清晰**：路由注册与业务逻辑完全解耦，方便维护。
2. **易于扩展**：新增路由时，不需要直接修改处理器逻辑。
3. **集中式配置**：所有路由在 `RouteRegistry` 中统一管理，便于查看。

这种方式兼具灵活性和可维护性，非常适合中大型项目的开发。
