/**
 * socket.io server namespace room示例代码
 */
const io = require('socket.io')(8080, {
  transports: ['websocket'],
});

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
    socket.join(sex); // 加入房间
    setTimeout(() => {
      // 发送给namespace内所有客户端
      chat.emit('chat', 'to all chat client');
      // 发送给man房间内所有客户端
      chat.to('man').emit('chat', 'to man room client');
      socket.to('man').emit('chat', 'client socket to man room client');
      socket.broadcast.to('man').emit('chat', 'client broadcast to man room client');
      clientIds.forEach((id) => {
        // 发送给特定用户，自动排除发送者
        socket.broadcast.to(id).emit('chat', `${socket.id} send to ${id}`);
      });
    }, 5000);
  });
