# rudanmaku-core
提供批量对bilibili直播间的监控服务
# 启动
- 使用配置启动： `rudanmaku-core config <config-path>`
- 单房间在localhost启动 `rudanmaku-core <roomid> <port?=10200>`
## 启动配置
配置是toml格式的，以下是一个例子
```toml
[net]
port = 10200
# 优先采用ipv4
ipv4 = [0,0,0,0]
# 如果没有ipv4，采用ipv6
ipv6 = [0,0,0,0,0,0,0,0]
# 如果都没有设置，默认采用ipv4 0.0.0.0

[[room]]
# 房间id长短都可以
roomid = 23840076
# channel 目前支持json|bincode
channel = ["json", "bincode"]

[[room]]
roomid = 5229
channel = ["json", "bincode"]
```


# 连接
地址 `ws://<ip>:<port>/<roomid>/<channel>`

如`ws://localhost:10200/477317922/json`
## json示例
![json示例](./.readme/example.png)
## 事件类型
参见[这个仓库](https://github.com/4t145/bilive-danmaku)
