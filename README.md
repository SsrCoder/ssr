# ssr

一个轻量命令行工具集合，提供若干实用的小工具：时间戳、JSON 高亮/查询、crontab 解析、URL 编码/解码、以及基于 AI 的翻译。

**安装与构建**
- **Clone & build**: 使用 Rust 构建

```
git clone https://github.com/SsrCoder/ssr.git
cd ssr
cargo build --release
```

- 可执行文件位于 `target/release/ssr`，也可以通过 `cargo run -- <subcommand>` 直接运行。

**命令总览**
- **`timestamp` / `ts`**: 打印当前时间戳或解析给定时间戳。
- **`json`**: 对 JSON 数据进行语法高亮显示并支持 JSONPath 查询与压缩输出。
- **`crontab` / `cron`**: 解析 crontab 表达式并展示字段说明与接下来 10 个触发时间。
- **`translate` / `trans`**: 基于已配置的 AI 提供者进行文本翻译（需要配置 AI 提供者）。
- **`url`**: URL 编码/解码。

**命令详细说明与示例**

**`timestamp [timestamp]`** (别名: `ts`)
- 描述: 若不带参数，输出当前 Unix 时间戳并拷贝到剪贴板；若带参数（秒或毫秒），解析该时间戳并显示：Unix、GMT、Local 以及相对于当前时间的相对量（如 "3天 2小时 前"）。
- 用法示例:

```
ssr timestamp
ssr ts 1699999999
ssr ts 1699999999000    # 毫秒级时间戳也支持
```

注意: 复制到系统剪贴板依赖终端环境与剪贴板支持（代码使用 `crossterm` 的 clipboard 功能）。

**`json [data] [--path <jsonpath>] [--compress]`**
- 描述: 解析并以终端着色的方式高亮显示 JSON。`data` 可选——若未传入，则从 `stdin` 读取。
- 选项:
  - `-p, --path <path>`: 对 JSON 使用 JSONPath 查询（基于 `serde_json_path`），参考 RFC 9535。
  - `-c, --compress`: 输出压缩（单行）JSON。
- 用法示例:

```
ssr json '{"a":1, "b": [1,2,3]}'
echo '{"a":1, "b":[1,2,3]}' | ssr json
ssr json '{"a":1, "b": [1,2,3]}' --path '$.b[0]'
ssr json '{"a":1}' --compress
```

输出使用 `syntect` 做语法高亮，适合在支持 24-bit 颜色的终端中查看。

**`crontab <expression>`** (别名: `cron`)
- 描述: 解析 crontab 表达式并打印每个字段的具体取值说明，同时列出接下来的 10 次执行时间。
- 细节: 如果传入的是 5 字段（常见的 "min hour dom month dow"），工具会自动在前面补充秒字段（`0`），以兼容 `cron` 库的 6 字段格式。
- 示例:

```
ssr crontab "*/5 * * * *"
ssr cron "0 0 * * 0"    # 6 字段或 5 字段都可（5 字段会自动补 0 秒）
```

**`translate <text> [--from <lang>] [--to <lang>]`** (别名: `trans`)
- 描述: 使用配置的 AI 提供者（OpenAI 兼容）进行翻译请求。
- 选项:
  - `-f, --from <lang>`: 源语言（可选），例如 `cn`、`en`。
  - `-t, --to <lang>`: 目标语言，默认 `CN`（即中文）。
- 运行方式: 翻译请求通过 openai 风格的客户端发出，运行时会读取配置文件中指定的 provider（见下）。
- 示例:

```
ssr translate "hello world" --to cn
ssr trans "你好" --to en
```

**`url <text> [--decode]`**
- 描述: URL 编码或解码文本。
- 选项:
  - `-d, --decode`: 解码而不是编码。
- 示例:

```
ssr url "a b"          # 输出: a%20b
ssr url "a%20b" --decode   # 输出: a b
```

**配置（AI 翻译）**
- 配置文件位置: `~/.config/ssr/init.lua`（遵循 XDG 配置目录）。
- 示例 `init.lua`:

```lua
return {
    ai = {
        default = "deepseek-chat",
        providers = {
            {
                enable = true,
                name = "deepseek-chat",
                key = "sk-xxxxxxxxxxxxxx",
                base_url = "https://api.deepseek.com/v1",
                model = "deepseek-chat",
            },
            {
                enable = true,
                name = "qwen-flash",
                key = "sk-xxxxxxxxxxxxxx",
                base_url = "https://dashscope.aliyuncs.com/compatible-mode/v1",
                model = "qwen-flash",
            },
        },
    },
    translate = {
        ai_provider = "qwen-flash",
    },
}
```

在配置中设置 `ai.providers` 列表并开启 `enable = true`，`translate.ai_provider` 指向你想使用的 provider 名称。
