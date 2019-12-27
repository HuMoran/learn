# Websocket & Socket.io 分享

## 为什么会有websocket

HTTP是一种无状态协议，客户端与服务器之间的HTTP连接是一种一次性连接，它限制每次连接只处理一个请求，当服务器返回本次请求的应答后便立即关闭连接，下次请求再重新建立连接。（http1.0 添加keep-alive保持长连接，http1.1 默认长连接， http 2.0 单一连接多路复用）

无状态协议导致服务器无法主动向客户端推送数据，客户端只能通过轮询（比如1秒一次），向服务器请求数据。弊端：增加服务器负荷，消息不及时

## websocket是什么

WebSocket是一种网络传输协议，可在单个TCP连接上进行全双工通信，位于OSI模型的应用层。

WebSocket是一种与HTTP不同的协议。两者都位于OSI模型的应用层，并且都依赖于传输层的TCP协议。 虽然它们不同，但RFC 6455规定：“WebSocket设计为通过80和443端口工作，以及支持HTTP代理和中介”，从而使其与HTTP协议兼容。 为了实现兼容性，WebSocket握手使用HTTP Upgrade头从HTTP协议更改为WebSocket协议。

WebSocket协议规范将ws（WebSocket）和wss（WebSocket Secure）定义为两个新的统一资源标识符（URI）方案，分别对应明文和加密连接。

```shell
ws://example.com/wsapi
wss://secure.example.com/wsapi
```

WebSocket 是独立的、创建在 TCP 上的协议（和http一样，是独立的应用层协议，tcp是传输层协议）, Websocket 通过 HTTP/1.1 协议的101状态码进行握手（握手数据通过http传输，建立连接后，和http没关系）。

101 Switching Protocols
服务器转换协议：服务器将遵从客户的请求转换到另外一种协议。

---

### websocket握手过程

客户端请求

```txt
GET / HTTP/1.1
Upgrade: websocket
Connection: Upgrade
Host: example.com
Origin: http://example.com
Sec-WebSocket-Key: sN9cRrP/n9NdMgdcy2VJFQ==
Sec-WebSocket-Version: 13
```

服务器回应

```txt
HTTP/1.1 101 Switching Protocols
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Accept: fFBooB7FAkLlXgRSz0BT3v4hq5s=
Sec-WebSocket-Location: ws://example.com/
```

字段说明
Connection必须设置Upgrade，表示客户端希望连接升级。
Upgrade字段必须设置Websocket，表示希望升级到Websocket协议。
Sec-WebSocket-Key是随机的字符串，服务器端会用这些数据来构造出一个SHA-1的信息摘要。把“Sec-WebSocket-Key”加上一个特殊字符串“258EAFA5-E914-47DA-95CA-C5AB0DC85B11”，然后计算SHA-1摘要，之后进行BASE-64编码，将结果做为“Sec-WebSocket-Accept”头的值，返回给客户端。如此操作，可以尽量避免普通HTTP请求被误认为Websocket协议。
Sec-WebSocket-Version 表示支持的Websocket版本。RFC6455要求使用的版本是13，之前草案的版本均应当弃用。
Origin字段是可选的，通常用来表示在浏览器中发起此Websocket连接所在的页面，类似于Referer。但是，与Referer不同的是，Origin只包含了协议和主机名称。
其他一些定义在HTTP协议中的字段，如Cookie等，也可以在Websocket中使用。

### websocket数据帧

```txt
Frame format:
    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    +-+-+-+-+-------+-+-------------+-------------------------------+
    |F|R|R|R| opcode|M| Payload len |    Extended payload length    |
    |I|S|S|S|  (4)  |A|     (7)     |             (16/64)           |
    |N|V|V|V|       |S|             |   (if payload len==126/127)   |
    | |1|2|3|       |K|             |                               |
    +-+-+-+-+-------+-+-------------+ - - - - - - - - - - - - - - - +
    |     Extended payload length continued, if payload len == 127  |
    + - - - - - - - - - - - - - - - +-------------------------------+
    |                               |Masking-key, if MASK set to 1  |
    +-------------------------------+-------------------------------+
    | Masking-key (continued)       |          Payload Data         |
    +-------------------------------- - - - - - - - - - - - - - - - +
    :                     Payload Data continued ...                :
    + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
    |                     Payload Data continued ...                |
    +---------------------------------------------------------------+
```

```txt
FIN： 1bit
　　表示此帧是否是消息的最后帧。第一帧也可能是最后帧。

RSV1，RSV2，RSV3： 各1bit
　　必须是0，除非协商了扩展定义了非0的意义。如果接收到非0，且没有协商扩展定义此值的意义，接收端必须使WebSocket连接失败。

Opcode： 4bit
　　定义了"Payload data"的解释。如果接收到未知的操作码，接收端必须使WebSocket连接失败。下面的值是定义了的。
　　%x0 表示一个后续帧
　　%x1 表示一个文本帧
　　%x2 表示一个二进制帧
　　%x3-7 为以后的非控制帧保留
　　%x8 表示一个连接关闭
　　%x9 表示一个ping
　　%xA 表示一个pong
　　%xB-F 为以后的控制帧保留

Mask： 1bit
　　定义了"Payload data"是否标记了。如果设为1，必须有标记键出现在masking-key，用来unmask "payload data"，见5.3节。所有从客户端发往服务器的帧必须把此位设为1。

Payload length： 7bit, 7 + 16bit, 7 + 64bit
　　"Payload data"的长度，字节单位。如果值是0-125，则是有效载荷长度。如果是126，接下来的2字节解释为16位无符号整数，作为有效载荷长度。如果127，接下来的8字节解释为64位无符号整数（最高位必须是0），作为有效载荷长度。多字节长度数值以网络字节序表示。注意，在任何情况下，必须用最小数量的字节来编码长度，例如，124字节长的字符串不能编码为序列126, 0, 124。有效载荷长度是"Extension data"的长度加上"Application data"的长度。"Extension data"的长度可能是0，在这种情况下，有效载荷长度是"Application data"的长度。

Masking-key：`0`或`4`字节
　　所有从客户端发往服务器的帧必须用`32`位值标记，此值在帧里。如果mask位设为1，此字段（`32`位值）出现，否则缺失。更多的信息在`5.3`节，客户端到服务器标记。

Payload data： (x + y)字节
　　"Payload data" 定义为"extension data" 后接 "application data"。

Extension data： x 字节
　　"Extension data"是0字节，除非协商了扩张。所有扩张必须指定"extension data"的长度，或者如何计算长度，如何使用扩展必须在打开握手时进行协商。如果有，"Extension data"包括在有效载荷长度。

Application data： y字节
任意"Application data"占据了帧的剩余部分，在"Extension data"之后。"Application data"的长度等于有效载荷长度减去"Extension data"的长度。
```

### 关闭状态码定义

```txt
1000：表示正常关闭，意味着连接建立的目的已完成。
1001：表示终端离开，例如服务器关闭或浏览器导航到其他页面。
1002：表示终端因为协议错误而关闭连接。
1003：表示终端因为接收到不能接受的数据而关闭（例如，只明白文本数据的终端可能发送这个，如果它接收到二进制消息）。
1004：保留。这个特定含义可能在以后定义。
1005：保留。且终端必须不在控制帧里设置作为状态码。它是指定给应用程序而非作为状态码使用的，用来指示没有状态码出现。
1006：同上。保留。且终端必须不在控制帧里设置作为状态码。它是指定给应用程序而非作为状态 码 使用的，用来指示连接非正常关闭，例如，没有发生或接收到关闭帧。
1007：表示终端因为接收到的数据没有消息类型而关闭连接。
1008：表示终端因为接收到的消息背离它的政策而关闭连接。这是一个通用的状态码，用在没有更合适的状态码或需要隐藏具体的政策细节时。
1009：表示终端因为接收到的消息太大以至于不能处理而关闭连接。
1010：表示客户端因为想和服务器协商一个或多个扩展，而服务器不在响应消息返回它（扩展）而关闭连接。需要的扩展列表应该出现在关闭帧的/reason/部分。注意，这个状态 码不是由服务器使用，因为它会导致WebSocket握手失败。
1011：表示服务器因为遇到非预期的情况导致它不能完成请求而关闭连接。
1015：保留，且终端必须不在控制帧里设置作为状态码。它是指定用于应用程序希望用状态 码来指示连接因为TLS握手失败而关闭。
```

## websocket协议实现

浏览器、ws、uws、socket.io、mqtt

uws性能最好[benchmark](https://github.com/uNetworking/pubsub-benchmark)
socket.io支持websocket协议传输数据，也支持轮询。使用WebSocket协议进行传输时，它会向每个数据包添加一些元数据，所以无法用ws的客户端直接连接socket.io server

[socket.io协议介绍](https://github.com/socketio/socket.io-protocol)

## socket.io介绍

主要功能
1、可靠，适用范围广（先通过长轮询，然后尝试切换到websocket）
2、自动重连
3、掉线检测
4、二进制传输
5、支持多路传输（多个命名空间隔离，但是共享一个TCP通信连接）
6、在命名空间下，支持房间机制
