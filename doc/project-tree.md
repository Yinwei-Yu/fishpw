以下是一个建议的 `pw` 密码本工具的项目结构组织方案：

```
pw/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
├── src/
│   ├── main.rs
│   ├── cli.rs             # CLI/UI 模块：命令行参数解析和交互式输入
│   ├── core.rs            # Core Logic 模块：核心业务逻辑，协调各模块
│   ├── data_store.rs      # Data Store 模块：SQLite 数据库操作
│   ├── encryption.rs      # Encryption 模块：加密、解密、KDF、IV 生成
│   ├── models.rs          # 数据结构定义（PasswordEntry, PasswordVaultMetadata）
│   ├── security.rs        # Security Utilities 模块：内存归零等安全辅助
│   └── utils.rs           # Utils 模块：日志初始化、路径管理、密码生成等通用工具
├── tests/
│   └── integration_tests.rs # 集成测试
├── .env                  # 环境变量文件 (例如，用于测试时指定临时数据库路径)
└── target/               # 编译输出目录 (由 cargo 自动生成)
```

### 结构说明：

1.  **`pw/` (根目录)**

      * `Cargo.toml`: Rust 项目的清单文件，定义了项目元数据、依赖项、特性等。
      * `Cargo.lock`: Cargo 自动生成的依赖锁定文件，确保构建的可重复性。
      * `README.md`: 项目的介绍文档，包含功能、安装、使用说明、安全声明等。
      * `.gitignore`: Git 版本控制忽略文件，防止提交不必要的文件（如 `target/` 目录、日志文件、敏感配置等）。
      * `.env`: 可选的环境变量文件，用于开发和测试时配置。

2.  **`src/` (源代码目录)**

      * `main.rs`: 程序的入口点。它将负责初始化日志系统，解析顶层命令行参数，并将控制权分发给 `cli.rs` 中的具体命令处理函数。
      * `cli.rs`:
          * 封装 `clap` 和 `dialoguer` 的使用。
          * 定义 `pw` 工具的子命令（`init`, `add`, `find`, `edit`, `delete`, `copy`, `set-master-password` 等）。
          * 处理用户的交互式输入和命令行输出。
          * 将用户请求（经过解析和交互后）传递给 `core.rs` 模块。
      * `core.rs`:
          * 实现核心业务逻辑，例如验证主密码、协调加密和数据存储操作。
          * 包含业务规则的实现（例如，密码生成逻辑，但具体生成算法可以在 `utils.rs` 中）。
          * 处理高级功能，如剪贴板操作的协调。
      * `data_store.rs`:
          * 负责与 SQLite 数据库的所有交互。
          * 定义数据库模式（表结构）。
          * 提供 `open_db`, `create_tables`, `insert_entry`, `get_entry_by_uuid`, `update_entry`, `delete_entry` 等 CRUD 操作函数。
          * 管理数据库连接池或单例。
      * `encryption.rs`:
          * 集中处理所有加密和解密操作。
          * 包含密钥派生函数（KDF，使用 `argon2`）。
          * 实现对称加解密（使用 `aes-gcm`）。
          * 负责生成安全的随机数（盐值、IV）。
      * `models.rs`:
          * 定义项目中使用到的所有数据结构，如 `PasswordEntry` 和 `PasswordVaultMetadata`。
          * 为这些结构体派生 `serde::Serialize` 和 `serde::Deserialize` 特性。
      * `security.rs`:
          * 包含与安全相关的辅助函数，特别是敏感数据在内存中归零 (`zeroize`) 的实现。
          * 可以包含其他安全最佳实践相关的辅助函数。
      * `utils.rs`:
          * 存放通用辅助函数，不属于上述任何特定模块。
          * **日志初始化**: `setup_logging()` 函数的实现。
          * **路径管理**: 获取跨平台用户目录的辅助函数。
          * **密码生成器**: 如果有独立的密码生成逻辑，可以放在这里。

3.  **`tests/` (集成测试目录)**

      * `integration_tests.rs`: 包含模拟命令行交互、验证端到端流程（例如：初始化 -\> 添加 -\> 查找 -\> 删除）的测试用例。这些测试将使用 `assert_cmd` 和 `tempfile` 来确保测试环境的隔离和输出的断言。

### 模块间职责和协作：

  * `main.rs` 是程序的入口，进行全局初始化（如日志），然后将控制权传递给 `cli.rs`。
  * `cli.rs` 负责用户输入输出，并根据用户命令调用 `core.rs` 中的相应业务逻辑。
  * `core.rs` 是业务逻辑的枢纽，它不直接与数据库或加密算法交互，而是协调 `data_store.rs` 和 `encryption.rs` 来完成任务。
  * `encryption.rs` 和 `data_store.rs` 提供底层服务，供 `core.rs` 调用。
  * `models.rs` 提供数据定义，供所有需要处理数据结构的地方使用。
  * `security.rs` 和 `utils.rs` 提供横切关注点和通用功能。