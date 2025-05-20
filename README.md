# Database Migration Tool

A powerful command-line tool for database migration and data merging operations, written in Rust.

## Features

- YAML-based configuration
- ODBC database connectivity
- Metadata collection and comparison
- Interactive decision making
- Progress tracking
- HTML and JSON reports
- Dry-run mode
- Batch processing
- Parallel execution

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/db-migrate-tool.git
cd db-migrate-tool

# Build the project
cargo build --release

# The binary will be available at target/release/db-migrate-tool
```

## Configuration

Create a `config.yaml` file with the following structure:

```yaml
dsn:
  origin:
    driver: "DM8 ODBC DRIVER"
    server: "172.16.117.71:15236"
    database: "JWAB"
    schema: "JWAB"
    username: "JWAB"
    password: "jhy123456"
  reference:
    driver: "DM8 ODBC DRIVER"
    server: "172.16.117.71:15236"
    database: "JWAB01"
    schema: "JWAB01"
    username: "JWAB01"
    password: "jhy123456"
  target:
    driver: "DM8 ODBC DRIVER"
    server: "172.16.117.71:15236"
    database: "JWAB02"
    schema: "JWAB02"
    username: "JWAB02"
    password: "jhy123456"

connection_pool:
  min_size: 1
  max_size: 10
  timeout_seconds: 30

batch:
  batch_size: 1000
  commit_frequency: 100

parallel:
  enabled: true
  thread_count: 4

log_level: "info"
```

## Usage

Basic usage:

```bash
db-migrate-tool -c config.yaml
```

With additional options:

```bash
# Run in dry-run mode (no actual changes)
db-migrate-tool -c config.yaml --dry-run

# Set log level
db-migrate-tool -c config.yaml --log-level debug

# Specify output directory for reports
db-migrate-tool -c config.yaml --output-dir ./reports
```

### Command Line Options

- `-c, --config`: Path to the configuration file (required)
- `--dry-run`: Run in simulation mode without making changes
- `-l, --log-level`: Set logging level (debug, info, warn, error)
- `-o, --output-dir`: Directory for report output (default: "reports")

## Reports

The tool generates two types of reports:

1. HTML Report (`migration_report_YYYYMMDD_HHMMSS.html`)
   - Summary of migration
   - Error and warning details
   - Decision records
   - Performance metrics

2. JSON Report (`migration_report_YYYYMMDD_HHMMSS.json`)
   - Machine-readable format
   - Complete migration details
   - Can be used for further analysis

## 中文说明

### 功能特点

- 基于 YAML 的配置文件
- ODBC 数据库连接
- 元数据收集和比较
- 交互式决策处理
- 进度跟踪
- HTML 和 JSON 报告
- 模拟运行模式
- 批量处理
- 并行执行

### 安装方法

```bash
# 克隆仓库
git clone https://github.com/yourusername/db-migrate-tool.git
cd db-migrate-tool

# 编译项目
cargo build --release

# 编译后的程序位于 target/release/db-migrate-tool
```

### 配置文件

创建 `config.yaml` 文件，结构如下：

```yaml
dsn:
  origin:
    driver: "DM8 ODBC DRIVER"
    server: "172.16.117.71:15236"
    database: "JWAB"
    schema: "JWAB"
    username: "JWAB"
    password: "jhy123456"
  reference:
    driver: "DM8 ODBC DRIVER"
    server: "172.16.117.71:15236"
    database: "JWAB01"
    schema: "JWAB01"
    username: "JWAB01"
    password: "jhy123456"
  target:
    driver: "DM8 ODBC DRIVER"
    server: "172.16.117.71:15236"
    database: "JWAB02"
    schema: "JWAB02"
    username: "JWAB02"
    password: "jhy123456"

connection_pool:
  min_size: 1
  max_size: 10
  timeout_seconds: 30

batch:
  batch_size: 1000
  commit_frequency: 100

parallel:
  enabled: true
  thread_count: 4

log_level: "info"
```

### 使用方法

基本用法：

```bash
db-migrate-tool -c config.yaml
```

高级选项：

```bash
# 模拟运行模式（不实际修改数据）
db-migrate-tool -c config.yaml --dry-run

# 设置日志级别
db-migrate-tool -c config.yaml --log-level debug

# 指定报告输出目录
db-migrate-tool -c config.yaml --output-dir ./reports
```

### 命令行参数

- `-c, --config`: 配置文件路径（必需）
- `--dry-run`: 模拟运行模式，不实际修改数据
- `-l, --log-level`: 设置日志级别（debug, info, warn, error）
- `-o, --output-dir`: 报告输出目录（默认：reports）

### 报告说明

工具生成两种类型的报告：

1. HTML 报告（`migration_report_YYYYMMDD_HHMMSS.html`）
   - 迁移摘要
   - 错误和警告详情
   - 决策记录
   - 性能指标

2. JSON 报告（`migration_report_YYYYMMDD_HHMMSS.json`）
   - 机器可读格式
   - 完整的迁移详情
   - 可用于进一步分析

## License

MIT License

