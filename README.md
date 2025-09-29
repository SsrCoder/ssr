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
