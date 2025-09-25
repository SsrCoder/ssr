# ssr

这是一个工具库项目

目前支持的工具：

- `ssr json <text>`
- `ssr timestamp`
- `ssr translate <text>`
- `ssr crontab <express>`

其中 translate 基于大模型翻译，需要配置大模型，新建文件 `~/.config/ssr/init.lua` 并添加如下配置：

```lua
return {
    ai = {
        enable = true,
        key = "", -- api key
        base_url = "", -- 兼容 openai api 的接口
        model = "", -- 使用的模型
    },
}
```
