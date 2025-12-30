# SafeLine SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/safeline-sdk.svg)](https://crates.io/crates/safeline-sdk)
[![Documentation](https://docs.rs/safeline-sdk/badge.svg)](https://docs.rs/safeline-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK for SafeLine WAF API，提供完整的 IP 组管理功能。

## 功能特性

- ✅ 完整的 IP 组 CRUD 操作
- ✅ 批量 IP 添加到组
- ✅ 搜索引擎蜘蛛组管理
- ✅ 通过链接获取 IP
- ✅ 异步 HTTP 客户端
- ✅ 类型安全的 API 接口
- ✅ 完善的错误处理

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
safeline-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 快速开始

```rust
use safeline_rs::{Client, IPGroupCreateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = Client::new("https://your-safeline-host:9443", "your-api-token");
    
    // 创建 IP 组
    let create_req = IPGroupCreateRequest {
        comment: Some("测试IP组".to_string()),
        ips: vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()],
        reference: Some("test-reference".to_string()),
    };
    
    let response = client.ip_group_create(&create_req).await?;
    println!("IP 组创建成功，ID: {}", response.data);
    
    Ok(())
}
```

## API 接口

### IP 组管理

#### 创建 IP 组
```rust
let create_req = IPGroupCreateRequest {
    comment: Some("注释".to_string()),
    ips: vec!["192.168.1.1".to_string()],
    reference: Some("引用标识".to_string()),
};
let response = client.ip_group_create(&create_req).await?;
```

#### 获取 IP 组列表
```rust
let list_req = IPGroupListRequest { top: Some(10) };
let response = client.ip_group_list(&list_req).await?;
println!("找到 {} 个 IP 组", response.total);
```

#### 获取 IP 组详情
```rust
let detail_req = IPGroupDetailRequest { id: 1 };
let response = client.ip_group_detail(&detail_req).await?;
```

#### 更新 IP 组
```rust
let update_req = IPGroupUpdateRequest {
    id: 1,
    comment: Some("更新的注释".to_string()),
    ips: Some(vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()]),
    ..Default::default()
};
let response = client.ip_group_update(&update_req).await?;
```

#### 删除 IP 组
```rust
let delete_req = IPGroupDeleteRequest { ids: vec![1, 2, 3] };
let response = client.ip_group_delete(&delete_req).await?;
```

### IP 组操作

#### 添加 IP 到组
```rust
let append_req = IPGroupAppendRequest {
    ip_group_ids: vec![1],
    ips: vec!["172.16.0.1".to_string()],
};
let response = client.ip_group_append(&append_req).await?;
```

#### 搜索引擎蜘蛛组
```rust
// 获取蜘蛛组 ID
let crawler_response = client.ip_group_crawler().await?;
println!("蜘蛛组 ID: {}", crawler_response.data);

// 更新蜘蛛 IP
client.ip_group_crawler_update().await?;
```

#### 通过链接获取 IP
```rust
let link_req = IPGroupLinkRequest {
    href: "https://example.com".to_string(),
};
let response = client.ip_group_link(&link_req).await?;
println!("找到 IP: {:?}", response.data.ips);
```

## 错误处理

SDK 提供了完善的错误处理机制：

```rust
use safeline_rs::{Client, error::{Error, error_codes, is_success}};

let client = Client::new("https://your-host:9443", "your-token");

match client.ip_group_list(&IPGroupListRequest { top: None }).await {
    Ok(response) => {
        if is_success(response.code) {
            println!("操作成功");
        }
    }
    Err(Error::ApiError { code, message }) => {
        match code {
            error_codes::AUTH_FAILED => println!("认证失败"),
            error_codes::NOT_FOUND => println!("资源不存在"),
            _ => println!("API 错误: {}", message),
        }
    }
    Err(e) => println!("其他错误: {}", e),
}
```

## 运行示例

```bash
# 设置环境变量
export SAFELINE_HOST="https://your-safeline-host:9443"
export SAFELINE_TOKEN="your-api-token"

# 运行示例
cargo run --example ipgroup_example

# 运行测试
cargo test

# 运行集成测试（需要真实的 SafeLine 实例）
cargo test --test integration_tests -- --ignored
```

## 开发

### 构建项目

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

### 生成文档

```bash
cargo doc --open
```

## API 文档

详细的 API 文档请参考 [SafeLine API 文档](./doc.json)。

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### v0.1.0
- 初始版本发布
- 实现完整的 IP 组管理 API
- 支持异步操作
- 完善的错误处理