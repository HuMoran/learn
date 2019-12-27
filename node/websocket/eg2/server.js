/**
 * socket.io server namespace示例代码
 */
const io = require('socket.io')(8080, {
  transports: ['websocket'],
});

const chat = io
  .of('/chat')
  .on('connection', (socket) => {
    console.log('chat id:', socket.id);
    socket.emit('chat_msg', '定向');
    chat.emit('chat_msg', 'chat namespace 广播');
  });

const news = io
  .of('/news')
  .on('connection', (socket) => {
    console.log('news id:', socket.id);
    socket.emit('news_msg', { news: 'item' });
  });
