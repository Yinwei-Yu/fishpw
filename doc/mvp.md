好的，这是一个最小实施方案（MVP），旨在让你在最短时间内搭建起 `pw` 密码本工具的最核心骨架，并在此过程中熟悉关键的第三方库。

这个 MVP 将专注于实现以下功能：

1.  **初始化密码库**: 首次运行，设置主密码，并创建空的 SQLite 数据库和存储加密元数据。
2.  **添加密码条目**: 通过交互式命令行输入，加密密码并存储到 SQLite 数据库。
3.  **查看密码条目**: 解锁密码库后，能够从数据库读取并解密一个密码条目，并显示其非敏感信息。

### `pw` 密码本工具：最小实施方案 (MVP)

**目标**: 快速验证核心安全机制、数据持久化和基本命令行交互流程。

#### 1. 核心功能点

* **项目初始化**:
    * 使用 `cargo new pw --bin` 创建项目。
    * 配置 `Cargo.toml`，添加所有必要的第三方库依赖。
* **日志系统**:
    * 集成 `fern`，实现日志写入文件，包含时间、文件路径、行号、级别、内容。
* **数据结构**:
    * 定义 `PasswordEntry` 和 `PasswordVaultMetadata` 结构体。
* **主密码设置与密码库初始化 (`pw init` 命令)**:
    * 当密码库文件不存在时，引导用户设置主密码。
    * 使用 `argon2` 从主密码派生加密密钥。
    * 生成随机盐值。
    * 创建 SQLite 数据库文件。
    * 在数据库中创建表来存储 `PasswordVaultMetadata`（包含盐值和 Argon2 参数）。
    * 在数据库中创建表来存储 `PasswordEntry`。
* **密码库解锁**:
    * 每次操作前，提示用户输入主密码。
    * 使用存储的盐值和 Argon2 参数验证主密码，派生出加密密钥。
* **添加密码条目 (`pw add` 命令)**:
    * 通过 `dialoguer` 交互式获取**服务名**、**账号**和**明文密码**。
    * 生成随机 IV。
    * 使用 `aes-gcm` 和派生密钥、IV 加密明文密码。
    * 将加密后的 `PasswordEntry`（包含 UUID、服务名、账号、加密密码、IV、时间戳）存储到 SQLite 数据库。
    * （可选）在内存中对明文密码进行 `zeroize`。
* **查看单个密码条目 (`pw get <UUID>` 命令)**:
    * 通过命令行参数获取 `UUID`。
    * 从 SQLite 数据库中根据 `UUID` 读取加密的 `PasswordEntry`。
    * 使用 `aes-gcm` 和派生密钥、存储的 IV 解密密码。
    * 显示解密后的**服务名、账号**和**明文密码**（仅为 MVP 演示，实际应用中应谨慎显示）。
    * （可选）在内存中对解密后的明文密码进行 `zeroize`。

#### 2. 所需第三方库（MVP 阶段重点）

在 MVP 阶段，你将主要接触和熟悉以下库：

* **`clap`**: 用于定义 `init`, `add`, `get` 命令。
* **`dialoguer`**: 用于主密码输入和添加密码时的交互式输入。
* **`rusqlite`**: 用于数据库的创建、表定义、数据插入和查询。
* **`aes-gcm`**: 用于实际的密码加密和解密。
* **`argon2`**: 用于主密码的密钥派生。
* **`rand`**: 用于生成安全的随机盐值和 IV。
* **`zeroize`**: 用于敏感数据（如明文密码、密钥）的内存归零。
* **`log` / `fern` / `chrono` / `dirs`**: 用于设置详细的日志系统。
* **`anyhow`**: 简化错误处理和传播。
* **`uuid`**: 用于生成密码条目的唯一标识符。

#### 3. MVP 开发指导（分步实施）

建议按照以下步骤进行，每一步都专注于一个或几个库的集成和功能实现：

**步骤 1: 项目骨架与依赖**

* 创建新的 Rust 项目：`cargo new pw --bin`
* 编辑 `Cargo.toml`，添加上述所有 MVP 阶段所需的依赖。

**步骤 2: 日志系统**

* 在 `src/utils.rs` 或 `src/main.rs` 中实现 `setup_logging()` 函数，配置 `fern` 将日志输出到文件，并包含时间、文件、行号等信息。
* 在 `main` 函数的开头调用 `setup_logging()`。
* 在代码中尝试使用 `info!`, `error!`, `debug!` 宏，验证日志文件是否正确生成和记录。
    * **熟悉库**: `log`, `fern`, `chrono`, `dirs`

**步骤 3: 数据结构与安全辅助**

* 在 `src/models.rs` 中定义 `PasswordEntry` 和 `PasswordVaultMetadata` 结构体，并派生 `serde::Serialize`, `serde::Deserialize`。
* 为 `PasswordEntry` 中存储明文密码的字段（如果暂时有）或解密后的密码（在内存中）实现 `Zeroize` 特性。
    * **熟悉库**: `serde`, `zeroize`

**步骤 4: 加密核心**

* 在 `src/encryption.rs` 模块中：
    * 实现从主密码派生密钥的函数（使用 `argon2`）。
    * 实现生成随机盐值和 IV 的函数（使用 `rand`）。
    * 实现使用 AES-GCM 加密和解密 `Vec<u8>` 的函数。
    * 编写简单的单元测试来验证加解密过程的正确性。
    * **熟悉库**: `aes-gcm`, `argon2`, `rand`

**步骤 5: 数据存储**

* 在 `src/data_store.rs` 模块中：
    * 实现初始化 SQLite 数据库的函数（创建 `password_entries` 表和 `vault_metadata` 表）。
    * 实现存储 `PasswordVaultMetadata` 的函数。
    * 实现存储单个 `PasswordEntry` 的函数。
    * 实现根据 UUID 查询单个 `PasswordEntry` 的函数。
    * **熟悉库**: `rusqlite`, `uuid`

**步骤 6: 命令行接口与核心逻辑**

* 在 `src/main.rs` 或 `src/cli.rs` 中：
    * 使用 `clap` 定义 `init`, `add`, `get` 子命令。
    * 实现 `init` 命令的逻辑：
        * 检查密码库文件是否存在。
        * 如果不存在，使用 `dialoguer` 引导用户输入并确认主密码。
        * 调用 `encryption` 模块派生密钥和生成盐值。
        * 调用 `data_store` 模块创建数据库并存储 `PasswordVaultMetadata`。
    * 实现 `add` 命令的逻辑：
        * 提示用户输入主密码解锁。
        * 使用 `dialoguer` 交互式获取服务名、账号、明文密码。
        * 调用 `encryption` 模块加密密码。
        * 调用 `data_store` 模块保存 `PasswordEntry`。
    * 实现 `get` 命令的逻辑：
        * 提示用户输入主密码解锁。
        * 通过 `clap` 获取 `UUID` 参数。
        * 调用 `data_store` 模块查询 `PasswordEntry`。
        * 调用 `encryption` 模块解密密码。
        * 打印解密后的信息。
    * **熟悉库**: `clap`, `dialoguer`, `anyhow`

**测试 MVP**:

* 运行 `cargo run -- init`，观察是否能成功创建密码库文件并设置主密码。
* 运行 `cargo run -- add`，尝试添加一个密码条目，验证是否能成功存储到数据库。
* 运行 `cargo run -- get <UUID>`，尝试获取并解密之前添加的密码，验证显示是否正确。
* 检查日志文件是否按预期记录了信息。

通过这个 MVP，你将对整个 `pw` 工具的核心流程和所选库的使用有一个直观的理解和实践经验。这将为你后续实现完整的增删改查、模糊查找、剪贴板等功能打下坚实的基础。