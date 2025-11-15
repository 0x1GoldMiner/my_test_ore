# 📱 MacBook 编译和使用指南

完整的在 macOS 系统上编译和使用 ORE Test2 Optimized 挖矿程序的详细指南。

---

## 📋 目录

1. [系统要求](#系统要求)
2. [安装前准备](#安装前准备)
3. [依赖安装](#依赖安装)
4. [项目编译](#项目编译)
5. [配置和使用](#配置和使用)
6. [常见问题](#常见问题)
7. [性能优化](#性能优化)

---

## 系统要求

### 最低配置
- **操作系统**: macOS 10.15 (Catalina) 或更高版本
- **处理器**: Intel Core i5 或 Apple Silicon (M1/M2/M3)
- **内存**: 8GB RAM（推荐 16GB）
- **硬盘**: 至少 5GB 可用空间
- **网络**: 稳定的互联网连接

### 推荐配置
- **操作系统**: macOS 13 (Ventura) 或更高版本
- **处理器**: Apple Silicon M2/M3 或 Intel Core i7
- **内存**: 16GB RAM 或更高
- **硬盘**: SSD，至少 10GB 可用空间

---

## 安装前准备

### 1. 安装 Xcode Command Line Tools

Xcode Command Line Tools 提供了编译所需的基础工具（如 git, clang 等）。

```bash
# 检查是否已安装
xcode-select -p

# 如果未安装，执行以下命令
xcode-select --install
```

安装过程中会弹出对话框，点击"安装"按钮，等待安装完成（约 5-10 分钟）。

### 2. 安装 Homebrew

Homebrew 是 macOS 的包管理器，用于安装各种开发工具。

```bash
# 安装 Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装后，根据提示将 Homebrew 添加到 PATH
# M1/M2/M3 芯片 Mac：
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
eval "$(/opt/homebrew/bin/brew shellenv)"

# Intel 芯片 Mac：
echo 'eval "$(/usr/local/bin/brew shellenv)"' >> ~/.zprofile
eval "$(/usr/local/bin/brew shellenv)"

# 验证安装
brew --version
```

---

## 依赖安装

### 1. 安装 Rust 工具链

```bash
# 安装 rustup（Rust 版本管理器）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 选择默认安装选项（输入 1 然后回车）

# 安装完成后，重新加载环境变量
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version

# 确保版本 >= 1.70
# 如果版本过低，更新 Rust
rustup update
```

### 2. 安装 OpenSSL（可选，推荐）

虽然项目使用了 `vendored` 特性可以自动编译 OpenSSL，但手动安装可以加快编译速度。

```bash
# 安装 OpenSSL
brew install openssl@3

# 设置环境变量（添加到 ~/.zshrc 或 ~/.bash_profile）
echo 'export PATH="/opt/homebrew/opt/openssl@3/bin:$PATH"' >> ~/.zshrc
echo 'export LDFLAGS="-L/opt/homebrew/opt/openssl@3/lib"' >> ~/.zshrc
echo 'export CPPFLAGS="-I/opt/homebrew/opt/openssl@3/include"' >> ~/.zshrc
echo 'export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl@3/lib/pkgconfig"' >> ~/.zshrc

# Intel Mac 路径不同：
# echo 'export PATH="/usr/local/opt/openssl@3/bin:$PATH"' >> ~/.zshrc
# echo 'export LDFLAGS="-L/usr/local/opt/openssl@3/lib"' >> ~/.zshrc
# echo 'export CPPFLAGS="-I/usr/local/opt/openssl@3/include"' >> ~/.zshrc

# 重新加载配置
source ~/.zshrc
```

### 3. 安装其他依赖

```bash
# 安装 pkg-config（用于查找库）
brew install pkg-config

# 安装 Git（通常已随 Xcode Command Line Tools 安装）
brew install git

# 验证安装
pkg-config --version
git --version
```

---

## 项目编译

### 1. 获取项目代码

如果你还没有项目代码：

```bash
# 克隆项目（替换为实际的仓库地址）
git clone https://github.com/YOUR_USERNAME/my_test_ore.git
cd my_test_ore

# 或者如果已经有代码，直接进入项目目录
cd /path/to/my_test_ore
```

### 2. 配置 ore-api 依赖

项目依赖 `ore-api`，已在 `Cargo.toml` 中配置为使用 Git 仓库：

```toml
ore-api = { git = "https://github.com/regolith-labs/ore", branch = "master" }
```

这个配置已经可以直接使用，无需修改。

**可选**：如果你想使用本地的 ore-api（用于开发调试）：

```bash
# 1. 克隆 ore 仓库到项目同级目录
cd ..
git clone https://github.com/regolith-labs/ore.git
cd my_test_ore

# 2. 修改 Cargo.toml，将 ore-api 改为本地路径
# ore-api = { path = "../ore/api" }
```

### 3. 编译项目

```bash
# 开发模式编译（快速，但性能较低）
cargo build

# 生产模式编译（推荐，性能优化）
cargo build --release

# 首次编译时间较长（约 5-15 分钟），请耐心等待
# 后续编译会快很多（增量编译）
```

**编译过程说明**：
1. Cargo 会自动下载所有依赖（约 100+ 个 crate）
2. 编译所有依赖项
3. 编译项目源码
4. 生成可执行文件

编译成功后，可执行文件位置：
- 开发模式: `./target/debug/ore-test2-optimized`
- 生产模式: `./target/release/ore-test2-optimized`

### 4. 验证编译

```bash
# 查看帮助信息
./target/release/ore-test2-optimized --help

# 应该看到类似输出：
# 基于 ore_refined 设计思路优化的 ORE 挖矿程序
#
# Usage: ore-test2-optimized --rpc <RPC> --keypair <KEYPAIR> <COMMAND>
# ...
```

---

## 配置和使用

### 1. 准备 Solana Keypair

#### 方式 A：使用现有 Keypair

如果你已经有 Solana 钱包：

```bash
# Phantom 钱包导出的私钥需要转换为 Solana CLI 格式
# 可以使用 solana-keygen 工具
brew install solana

# 从助记词恢复（如果有）
solana-keygen recover -o ~/my-keypair.json

# 或者导入现有的 keypair 文件
cp /path/to/your/keypair.json ~/my-keypair.json
chmod 600 ~/my-keypair.json
```

#### 方式 B：创建新 Keypair

```bash
# 安装 Solana CLI
brew install solana

# 创建新钱包
solana-keygen new -o ~/my-keypair.json

# 记录助记词和公钥！非常重要！

# 查看公钥
solana-keygen pubkey ~/my-keypair.json
```

### 2. 准备 RPC 节点

你需要一个 Solana RPC 节点 URL。有以下选择：

#### 免费 RPC（适合测试）
- **Solana 公共节点**: `https://api.mainnet-beta.solana.com`
  - 限制：较慢，有请求速率限制

#### 付费 RPC（推荐生产使用）
- **Helius**: https://helius.dev
  - 免费额度 + 付费计划
  - 注册后获取 RPC URL

- **QuickNode**: https://quicknode.com
  - 免费试用 + 付费计划

- **Alchemy**: https://alchemy.com
  - 免费额度 + 付费计划

### 3. 基础使用

#### 查看余额

```bash
./target/release/ore-test2-optimized \
  --rpc https://api.mainnet-beta.solana.com \
  --keypair ~/my-keypair.json \
  balance
```

#### 查看挖矿状态

```bash
./target/release/ore-test2-optimized \
  --rpc https://api.mainnet-beta.solana.com \
  --keypair ~/my-keypair.json \
  status
```

#### 查看 Board 信息

```bash
./target/release/ore-test2-optimized \
  --rpc https://api.mainnet-beta.solana.com \
  --keypair ~/my-keypair.json \
  board
```

#### 领取奖励

```bash
./target/release/ore-test2-optimized \
  --rpc https://api.mainnet-beta.solana.com \
  --keypair ~/my-keypair.json \
  claim
```

### 4. 自动挖矿

#### 方式 A：阈值算法（适合新手）

```bash
./target/release/ore-test2-optimized \
  --rpc https://your-rpc-url.com \
  --keypair ~/my-keypair.json \
  auto-threshold \
  --amount-sol 0.01 \
  --threshold-sol 0.01 \
  --min-squares 12 \
  --pick-squares 5 \
  --remaining-slots 15
```

**参数说明**：
- `--amount-sol 0.01`: 每个格子部署 0.01 SOL
- `--threshold-sol 0.01`: 只选择当前部署量 < 0.01 SOL 的格子
- `--min-squares 12`: 至少有 12 个格子满足条件才部署
- `--pick-squares 5`: 从满足条件的格子中选择最少的 5 个
- `--remaining-slots 15`: 剩余 15 个 slot 时开始部署

#### 方式 B：最优化算法（自动计算阈值）

```bash
./target/release/ore-test2-optimized \
  --rpc https://your-rpc-url.com \
  --keypair ~/my-keypair.json \
  auto-optimized \
  --amount-sol 0.01 \
  --min-squares 12 \
  --pick-squares 5 \
  --remaining-slots 15
```

最优化算法会自动计算阈值：`threshold = (total_deployed * 0.036) - 0.005`

### 5. 使用环境变量（可选，更方便）

```bash
# 添加到 ~/.zshrc 或 ~/.bash_profile
export ORE_RPC_URL="https://your-rpc-url.com"
export ORE_KEYPAIR="$HOME/my-keypair.json"

# 重新加载
source ~/.zshrc

# 创建别名方便使用
alias ore="$HOME/my_test_ore/target/release/ore-test2-optimized --rpc $ORE_RPC_URL --keypair $ORE_KEYPAIR"

# 之后可以简化命令：
ore balance
ore status
ore auto-optimized --amount-sol 0.01 --min-squares 12 --pick-squares 5
```

### 6. 后台运行（推荐）

使用 `nohup` 或 `screen` 在后台运行挖矿程序：

#### 方式 A：使用 nohup

```bash
nohup ./target/release/ore-test2-optimized \
  --rpc https://your-rpc-url.com \
  --keypair ~/my-keypair.json \
  auto-optimized \
  --amount-sol 0.01 \
  --min-squares 12 \
  --pick-squares 5 \
  > ore-miner.log 2>&1 &

# 查看进程
ps aux | grep ore-test2-optimized

# 查看日志
tail -f ore-miner.log

# 停止进程
kill $(pgrep -f ore-test2-optimized)
```

#### 方式 B：使用 screen

```bash
# 安装 screen
brew install screen

# 创建新会话
screen -S ore-miner

# 运行挖矿程序
./target/release/ore-test2-optimized \
  --rpc https://your-rpc-url.com \
  --keypair ~/my-keypair.json \
  auto-optimized \
  --amount-sol 0.01 \
  --min-squares 12 \
  --pick-squares 5

# 分离会话（保持后台运行）：按 Ctrl+A 然后按 D

# 重新连接会话
screen -r ore-miner

# 列出所有会话
screen -ls

# 终止会话
screen -X -S ore-miner quit
```

---

## 常见问题

### 编译问题

#### Q1: 编译时出现 OpenSSL 错误

```
error: failed to run custom build command for `openssl-sys`
```

**解决方案**：

```bash
# 方案 1：安装 OpenSSL
brew install openssl@3

# 设置环境变量
export OPENSSL_DIR=/opt/homebrew/opt/openssl@3  # M1/M2/M3
# 或
export OPENSSL_DIR=/usr/local/opt/openssl@3    # Intel

# 重新编译
cargo clean
cargo build --release
```

项目已经在 `Cargo.toml` 中启用了 `vendored` 特性，理论上不需要手动安装 OpenSSL。如果仍有问题，使用上述方案。

#### Q2: 编译时出现 "linker `cc` not found"

**解决方案**：

```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 验证
xcode-select -p
```

#### Q3: Cargo 下载依赖很慢

**解决方案 - 使用国内镜像源**：

```bash
# 创建或编辑 ~/.cargo/config.toml
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
EOF

# 重新编译
cargo clean
cargo build --release
```

#### Q4: Apple Silicon (M1/M2/M3) 编译问题

**解决方案**：

```bash
# 确保使用 ARM64 架构的 Homebrew
which brew
# 应该显示 /opt/homebrew/bin/brew

# 如果不是，重新安装 Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 确保 Rust 使用正确的目标
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

### 运行问题

#### Q5: "permission denied" 错误

**解决方案**：

```bash
# 给予执行权限
chmod +x ./target/release/ore-test2-optimized

# 确保 keypair 文件权限正确
chmod 600 ~/my-keypair.json
```

#### Q6: RPC 连接超时

**解决方案**：

1. 检查网络连接
2. 使用更快的 RPC 节点（付费 RPC）
3. 增加重试次数（程序已内置重试机制）

#### Q7: "insufficient funds" 错误

**解决方案**：

```bash
# 检查账户余额
solana balance ~/my-keypair.json

# 需要有足够的 SOL 用于：
# 1. 部署费用（每轮 amount-sol * pick-squares）
# 2. 交易费用（约 0.000005 SOL 每次）
# 3. 账户租金（约 0.002 SOL）

# 转入 SOL 到你的钱包地址
# 查看地址：
solana-keygen pubkey ~/my-keypair.json
```

#### Q8: 挖矿程序不部署

**可能原因**：
1. 满足条件的格子数量不够（`min-squares` 设置太高）
2. 阈值设置不合理
3. 网络延迟导致错过时机

**解决方案**：

```bash
# 降低 min-squares
--min-squares 8  # 从 12 改为 8

# 调整阈值
--threshold-sol 0.02  # 提高阈值

# 增加剩余 slots 阈值，提前部署
--remaining-slots 20  # 从 15 改为 20
```

---

## 性能优化

### 1. 编译优化

#### Release 模式编译（必须）

```bash
# 始终使用 --release 模式
cargo build --release

# Release 模式比 Debug 模式快 10-100 倍
```

#### 自定义编译优化

编辑 `Cargo.toml`，添加优化配置：

```toml
[profile.release]
opt-level = 3           # 最高优化级别
lto = true              # 链接时优化
codegen-units = 1       # 更好的优化，但编译较慢
strip = true            # 移除调试符号，减小文件大小
```

重新编译：

```bash
cargo clean
cargo build --release
```

### 2. 运行优化

#### 选择高性能 RPC

- 使用付费 RPC 服务（Helius, QuickNode, Alchemy）
- 选择地理位置接近的 RPC 节点
- 使用专用 RPC 而非公共 RPC

#### 优化挖矿参数

```bash
# 推荐参数组合（根据网络情况调整）

# 激进策略（更多部署）
--amount-sol 0.02 \
--min-squares 8 \
--pick-squares 8 \
--remaining-slots 20

# 保守策略（更少部署，更高成功率）
--amount-sol 0.01 \
--min-squares 12 \
--pick-squares 3 \
--remaining-slots 15
```

#### 网络优化

```bash
# 使用有线网络而非 Wi-Fi
# 关闭其他占用带宽的应用
# 使用 VPN 可能影响延迟，建议测试
```

### 3. 系统优化

#### 禁用睡眠模式

```bash
# 防止 Mac 进入睡眠
caffeinate -i ./target/release/ore-test2-optimized \
  --rpc https://your-rpc-url.com \
  --keypair ~/my-keypair.json \
  auto-optimized \
  --amount-sol 0.01 \
  --min-squares 12 \
  --pick-squares 5

# 或者修改系统设置：
# 系统偏好设置 -> 电池 -> 防止电脑自动进入睡眠（勾选）
```

#### 监控资源使用

```bash
# 使用 Activity Monitor（活动监视器）
# 应用程序 -> 实用工具 -> 活动监视器

# 或使用命令行
top -o cpu
# 查找 ore-test2-optimized 进程

# 内存使用
ps aux | grep ore-test2-optimized
```

### 4. Apple Silicon 优化

如果你使用 M1/M2/M3 芯片的 Mac：

```bash
# 确保编译为原生 ARM64 程序
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# 可执行文件位置
./target/aarch64-apple-darwin/release/ore-test2-optimized

# 验证架构
file ./target/release/ore-test2-optimized
# 应该显示：Mach-O 64-bit executable arm64
```

---

## 安全建议

### 1. 保护 Keypair

```bash
# 设置严格的文件权限
chmod 600 ~/my-keypair.json

# 备份 keypair（加密存储）
# 可以使用 macOS 的磁盘工具创建加密磁盘映像

# 永远不要：
# - 将 keypair 上传到 GitHub
# - 通过不安全的通道传输 keypair
# - 将 keypair 存储在云盘（除非加密）
```

### 2. 使用专用钱包

```bash
# 创建专门用于挖矿的钱包
# 不要在挖矿钱包中存储大量资金
# 定期将收益转移到冷钱包
```

### 3. 监控活动

```bash
# 定期检查余额和交易
solana balance ~/my-keypair.json

# 在 Solana Explorer 查看交易历史
# https://explorer.solana.com/address/YOUR_ADDRESS
```

---

## 进阶使用

### 1. 创建启动脚本

创建 `start-miner.sh`：

```bash
#!/bin/bash

# 配置
RPC_URL="https://your-rpc-url.com"
KEYPAIR="$HOME/my-keypair.json"
LOG_FILE="$HOME/ore-miner.log"

# 进入项目目录
cd "$HOME/my_test_ore" || exit

# 启动挖矿
./target/release/ore-test2-optimized \
  --rpc "$RPC_URL" \
  --keypair "$KEYPAIR" \
  auto-optimized \
  --amount-sol 0.01 \
  --min-squares 12 \
  --pick-squares 5 \
  --remaining-slots 15 \
  2>&1 | tee -a "$LOG_FILE"
```

使用：

```bash
# 添加执行权限
chmod +x start-miner.sh

# 运行
./start-miner.sh
```

### 2. 使用 launchd 开机自启（可选）

创建 `~/Library/LaunchAgents/com.ore.miner.plist`：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.ore.miner</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/YOUR_USERNAME/start-miner.sh</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/Users/YOUR_USERNAME/ore-miner-stdout.log</string>
    <key>StandardErrorPath</key>
    <string>/Users/YOUR_USERNAME/ore-miner-stderr.log</string>
</dict>
</plist>
```

加载服务：

```bash
# 替换 YOUR_USERNAME
sed -i '' 's/YOUR_USERNAME/YOUR_ACTUAL_USERNAME/g' ~/Library/LaunchAgents/com.ore.miner.plist

# 加载
launchctl load ~/Library/LaunchAgents/com.ore.miner.plist

# 查看状态
launchctl list | grep ore.miner

# 卸载
launchctl unload ~/Library/LaunchAgents/com.ore.miner.plist
```

### 3. 日志分析

```bash
# 实时查看日志
tail -f ore-miner.log

# 查看错误
grep -i error ore-miner.log

# 查看成功的部署
grep "部署完成" ore-miner.log

# 统计部署次数
grep -c "部署完成" ore-miner.log
```

---

## 更新和维护

### 1. 更新项目代码

```bash
# 拉取最新代码
git pull origin main  # 或你的分支名

# 重新编译
cargo clean
cargo build --release
```

### 2. 更新 Rust 工具链

```bash
# 更新 rustup 和 Rust
rustup update

# 查看版本
rustc --version
cargo --version
```

### 3. 更新依赖

```bash
# 更新 Cargo.lock
cargo update

# 重新编译
cargo build --release
```

---

## 参考资源

### 官方文档
- [Rust 官方文档](https://www.rust-lang.org/learn)
- [Solana 文档](https://docs.solana.com/)
- [ORE 项目](https://github.com/regolith-labs/ore)

### 社区资源
- [Rust 中文社区](https://rust.cc/)
- [Solana 中文](https://solana.org.cn/)

### 项目文档
- [README.md](./README.md) - 项目概述和功能介绍
- [QUICKSTART.md](./QUICKSTART.md) - 快速开始指南
- [OPTIMIZATION_GUIDE.md](./OPTIMIZATION_GUIDE.md) - 优化说明

---

## 总结

### 快速上手流程

1. **安装依赖** (15-30 分钟)
   ```bash
   xcode-select --install
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   brew install openssl@3 pkg-config solana
   ```

2. **编译项目** (5-15 分钟)
   ```bash
   cd my_test_ore
   cargo build --release
   ```

3. **配置钱包**
   ```bash
   solana-keygen new -o ~/my-keypair.json
   # 保存助记词！
   ```

4. **开始挖矿**
   ```bash
   ./target/release/ore-test2-optimized \
     --rpc https://api.mainnet-beta.solana.com \
     --keypair ~/my-keypair.json \
     auto-optimized \
     --amount-sol 0.01 \
     --min-squares 12 \
     --pick-squares 5
   ```

---

**祝你挖矿顺利！** 🚀💎

有问题请参考 [常见问题](#常见问题) 或查看项目的其他文档。
