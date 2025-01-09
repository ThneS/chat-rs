# sql_info

## 连接字符串

```shell
postgres://postgres:postgrespw@localhost:55000/postgres
```

## 表结构

```rust
struct Chat {
    id: usize,
    name: String,
    description: String,
    r#type: ChatType,
    users: Vec<usize>,
    notification: usize,
    created_at: DateTime<Utc>,
}
struct ChatUser {
    id: usize,
    name: String,
    email: String,
}

impl User {
  pub fn avatar(&self) -> String {
      ...
  }
}

struct Message {
    user_id: usize, // 4b (32bit), avatar, name
    chat_id: usize,
    content: MessageContent,
    created_at: DateTime<Utc>,
}

struct MessageId(usize);
struct ChatId(usize);
struct UserId(usize);
```
