## 简述

协议符合restful api规范。使用http(s)+json格式的报文。

## url前缀

url在描述时为（举例如下）:

    POST .../heart_beat

...表示前缀，目前前缀为"/qc/api/v1", 所以真实url等同于:

    POST http:xxxx:10010/qc/api/v1/heart_beat

## 响应状态码

| 值   | 定义                    | 备注                             |
| --- | --------------------- | ------------------------------ |
| 200 | OK                    | 请求成功并返回数据（适用于 GET、PUT、DELETE）。 |
| 201 | Created               | 成功创建资源（适用于 POST）。              |
| 400 | Bad Request           | 请求无效，通常是由于请求参数错误。              |
| 401 | Unauthorized          | 未授权，需要认证（如缺少或无效的 API Token）。   |
| 404 | Not Found             | 请求的资源不存在。                      |
| 500 | Internal Server Error | 服务器内部错误，通常由服务器端问题引起。           |

## Error对象

如果执行时发生错误或者异常，http头部表示错误码，具体错误信息由Error对象表示。
错误码定义见后面的错误码定义
当出错时，响应不再携带正常报文，而是"Error对象"。例如:
HTTP/1.1 401 Unauthorized

{
  "errno": 101,
  "errmsg": "account auth failed"
}

## 接口定义

**认证**

- 请求
  post .../auth

| 参数          | 类型     | 说明         |
| ----------- | ------ | ---------- |
| nick_name   | string | 昵称         |
| encrypt_str | string | key加密后的字符串 |

- 响应

| 参数    | 类型     | 说明                             |
| ----- | ------ | ------------------------------ |
| token | string | 令牌，后续所有和服务端的通信都需要带这个令牌，以表明合法身份 |

## 错误码定义

| 值   | 错误消息                  | 说明   | http响应码 |
| --- | --------------------- | ---- | ------- |
| 101 | Authentication failed | 鉴权失败 | 401     |
