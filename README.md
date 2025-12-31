# SafeLine SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/safeline-sdk.svg)](https://crates.io/crates/safeline-sdk)
[![Documentation](https://docs.rs/safeline-sdk/badge.svg)](https://docs.rs/safeline-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK for SafeLine WAF API，提供完整的 IP 组管理功能。

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

## 接口适配进度

### IP 组管理 (IPGroup)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| ip_group_list | GET /open/ip_group | ✅ 已实现 | ✅ 已测试 |
| ip_group_create | POST /open/ip_group | ✅ 已实现 | ✅ 已测试 |
| ip_group_detail | GET /open/ip_group/{id} | ✅ 已实现 | ✅ 已测试 |
| ip_group_update | PUT /open/ip_group/{id} | ✅ 已实现 | ✅ 已测试 |
| ip_group_delete | DELETE /open/ip_group | ✅ 已实现 | ✅ 已测试 |
| ip_group_append | POST /open/ip_group/append | ✅ 已实现 | ✅ 已测试 |
| ip_group_crawler | GET /open/ip_group/crawler | ✅ 已实现 | ✅ 已测试 |
| ip_group_crawler_update | PUT /open/ip_group/crawler | ✅ 已实现 | ✅ 已测试 |
| ip_group_link | GET /open/ip_group/link | ✅ 已实现 | ✅ 已测试 |
| ip_group_create_by_link | POST /open/ip_group/link | ✅ 已实现 | ✅ 已测试 |

### ACL 管理 (ACL)

| 方法 | 端点 | 状态 | 测试 | 备注 |
|------|------|------|------|------|
| get_acl_logs | GET /commercial/record/export | ✅ 已实现 | ⚠️ 需要许可证 | 商业版功能 |
| acl_relieve | PUT /open/acl/relieve | ✅ 已实现 | ⚠️ 需要许可证 | 依赖 get_acl_logs |
| list_acl_records | GET /open/records/acl | ✅ 已实现 | ✅ 已测试 | |
| get_website_acl | GET /open/site/{id}/acl | ✅ 已实现 | ✅ 已测试 | |
| set_website_acl | PUT /open/site/{id}/acl | ✅ 已实现 | ✅ 已测试 | |
| delete_website_acl_rule | DELETE /open/site/{id}/acl/{rule_id} | ✅ 已实现 | ⏳ 待测试 | |

### 网站管理 (Site)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| list_websites | GET /open/site | ✅ 已实现 | ✅ 已测试 |
| create_website | POST /open/site | ✅ 已实现 | ✅ 已测试 |
| get_website_detail | GET /open/site/{id} | ✅ 已实现 | ✅ 已测试 |
| update_website | PUT /open/site/{id} | ✅ 已实现 | ✅ 已测试 |
| delete_websites | DELETE /open/site | ✅ 已实现 | ✅ 已测试 |
| put_website_basic_info | PUT /open/site/{id}/basic_info | ✅ 已实现 | ✅ 已测试 |
| put_website_defense | PUT /open/site/{id}/defense | ✅ 已实现 | ✅ 已测试 |
| put_website_mode | PUT /open/site/mode | ✅ 已实现 | ✅ 已测试 |
| put_website_chaos | PUT /open/site/{id}/chaos | ✅ 已实现 | ✅ 已测试 |
| put_website_challenge | PUT /open/site/{id}/challenge | ✅ 已实现 | ✅ 已测试 |
| set_website_waiting_room | PUT /open/site/{id}/waiting_room | ✅ 已实现 | ✅ 已测试 |
| list_website_groups | GET /open/site/group | ✅ 已实现 | ✅ 已测试 |
| create_website_group | POST /open/site/group | ✅ 已实现 | ✅ 已测试 |
| update_website_group | PUT /open/site/group/{id} | ✅ 已实现 | ✅ 已测试 |
| delete_website_group | DELETE /open/site/group/{id} | ✅ 已实现 | ✅ 已测试 |
| sort_website_groups | PUT /open/site/group/{id}/sort | ✅ 已实现 | ✅ 已测试 |
| sort_websites | PUT /open/site/sort | ✅ 已实现 | ✅ 已测试 |
| group_switch | PUT /open/site/group/switch | ✅ 已实现 | ✅ 已测试 |
| get_nginx_config | GET /open/site/{id}/nginx_config | ✅ 已实现 | ✅ 已测试 |
| update_nginx_config | PUT /open/site/{id}/nginx_config | ✅ 已实现 | ✅ 已测试 |
| website_health_check | PUT /open/site/{id}/health_check | ✅ 已实现 | ✅ 已测试 |

### 策略管理 (Policy)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| list_policies | GET /open/policy | ✅ 已实现 | ⏳ 待测试 |
| create_policy | POST /open/policy | ✅ 已实现 | ⏳ 待测试 |
| get_policy_detail | GET /open/policy/{id} | ✅ 已实现 | ⏳ 待测试 |
| update_policy | PUT /open/policy/{id} | ✅ 已实现 | ⏳ 待测试 |
| delete_policy | DELETE /open/policy/{id} | ✅ 已实现 | ⏳ 待测试 |
| switch_policy | PUT /open/policy/{id}/switch | ✅ 已实现 | ⏳ 待测试 |

### 记录管理 (Record)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| list_detect_logs | GET /open/records/detect | ✅ 已实现 | ⏳ 待测试 |
| list_record_events | GET /open/records/event | ✅ 已实现 | ⏳ 待测试 |
| list_acl_records | GET /open/records/acl | ✅ 已实现 | ✅ 已测试 |

### 证书管理 (Cert)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| list_certs | GET /open/cert | ✅ 已实现 | ⏳ 待测试 |
| get_cert_detail | GET /open/cert/{id} | ✅ 已实现 | ⏳ 待测试 |
| create_cert | POST /open/cert | ✅ 已实现 | ⏳ 待测试 |
| update_cert | PUT /open/cert/{id} | ✅ 已实现 | ⏳ 待测试 |
| delete_cert | DELETE /open/cert/{id} | ✅ 已实现 | ⏳ 待测试 |

### JA4 指纹 (JA4)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| list_ja4 | GET /open/ja4 | ✅ 已实现 | ⏳ 待测试 |

### 统计信息 (Stat)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| get_dashboard_user_counts | GET /open/stat/dashboard/user_counts | ✅ 已实现 | ⏳ 待测试 |
| get_dashboard_trend | GET /open/stat/dashboard/trend | ✅ 已实现 | ⏳ 待测试 |
| get_basic_access | GET /open/stat/basic/access | ✅ 已实现 | ⏳ 待测试 |
| get_basic_attack | GET /open/stat/basic/attack | ✅ 已实现 | ⏳ 待测试 |

### 认证管理 (Auth)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| get_csrf_token | GET /open/auth/csrf_token | ✅ 已实现 | ⏳ 待测试 |
| login | POST /open/auth/login | ✅ 已实现 | ⏳ 待测试 |
| tfa | POST /open/auth/tfa | ✅ 已实现 | ⏳ 待测试 |
| get_auth_token | GET /open/auth/token | ✅ 已实现 | ⏳ 待测试 |

### 系统管理 (System)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| get_system_info | GET /open/system/info | ✅ 已实现 | ⏳ 待测试 |
| get_system_status | GET /open/system/status | ✅ 已实现 | ⏳ 待测试 |
| get_system_version | GET /open/system/version | ✅ 已实现 | ⏳ 待测试 |

### 其他功能 (Other)

| 方法 | 端点 | 状态 | 测试 |
|------|------|------|------|
| get_alarm_config | GET /open/alarm/config | ✅ 已实现 | ⏳ 待测试 |
| update_alarm_config | PUT /open/alarm/config | ✅ 已实现 | ⏳ 待测试 |

### 图例说明

- ✅ 已实现 - API 接口已实现
- ⏳ 待测试 - API 已实现但尚未测试
- ⚠️ 需要许可证 - 需要商业版许可证才能使用
- 🚫 不支持 - 暂不支持该接口

### 测试覆盖情况

- **已测试模块**: IPGroup, ACL (部分), Site
- **待测试模块**: Policy, Record (部分), Cert, JA4, Stat, Auth, System, Other
- **测试通过率**: 100% (已测试接口)

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