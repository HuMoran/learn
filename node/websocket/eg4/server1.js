/**
 * socket.io server redis adapter 示例代码
 * 负载均衡时，注意粘性会话（如：nginx基于ip hash路由）
 */
const redis = require('socket.io-redis');
const io = require('socket.io')(8080, {
  transports: ['websocket'],
});

io.adapter(redis({ host: '127.0.0.1' }));

const clientIds = [];

const chat = io
  .of('/chat')
  .on('connection', (socket) => {
    clientIds.push(socket.id);
    const { sex } = socket.handshake.query;
    if (sex !== 'man' && sex !== 'woman') {
      socket.disconnect(true);
      return;
    }
    socket.join('chat', () => {
      socket.broadcast.to('chat').emit('chat', `${socket.id} join chat room`);
    }); // 加入房间
  });
