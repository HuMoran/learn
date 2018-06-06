### Fetch
cocos打包原生app时，可能会出现fetch不能使用情况，这时需要添加一个[fetch-polyfill](./fetch-polyfill.js)。把这个文件添加到cocos项目就行。
### Websocket
* cocos游戏可以通过websocket建立长连接进行通信，ws客户端已经内置与cocos中。
* ws服务器库推荐：[µWS](https://github.com/uNetworking/uWebSockets)和[ws](https://github.com/websockets/ws),都支持nodejs，不过µWS是C++写的性能超强库。
* 使用wss通信时，有几个问题：
1. 打包Android时，可能会出现证书验证不通过情况，这时需要把证书放入cocos项目中，连接wss时指定证书。
```js
// 如果证书是官方注册申请的，可以从浏览器里面把自己注册的证书服务商的证书导出来使用。
const wss = new websocket('wss://xxx.xxx.xxx, null, cc.url.raw('证书文件'));
```