-- 使用 PostGresql 16 创建数据库
/* struct Chat {
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

struct Message {
    user_id: usize, // 4b (32bit), avatar, name
    chat_id: usize,
    content: MessageContent,
    created_at: DateTime<Utc>,
}
*/

CREATE TYPE CHAT_TYPE AS ENUM
( 'single', 'group', 'channel');

CREATE TABLE
IF NOT EXISTS CHAT
(
    id SERIAL PRIMARY KEY,
    name VARCHAR
(255) NOT NULL,
    description VARCHAR
(255) NOT NULL,
    r#type CHAT_TYPE NOT NULL,
    users INT NOT NULL,
    notification VARCHAR
(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE
IF NOT EXISTS CHAT_USER
(
    id SERIAL PRIMARY KEY,
    name VARCHAR
(255) NOT NULL,
    email VARCHAR
(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_chat_user_email ON CHAT_USER(email);


CREATE TABLE
IF NOT EXISTS MESSAGE
(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    chat_id INT NOT NULL,
    content VARCHAR
(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
