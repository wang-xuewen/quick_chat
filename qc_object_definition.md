# 对象定义

QcMessage
| 字段名   | 类型       | 说明   |
|-------|-----------|-------|
| cmd | string     | 命令代码 |
| nick_name | string     | 用户昵称 |
| message | string     | 用户发送的消息或命令参数对象序列化后的字符串 |
| token | string     | 令牌，激活后可取得令牌，鉴权用 |

## QcMessage's cmd定义
| 命令代码   | 说明   |
|-------|-------|
| 00000 |  客户端发送聊天消息到服务端 |
