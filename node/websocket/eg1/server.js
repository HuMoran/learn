/**
 * socket.io server示例代码
 */
const io = require('socket.io')(8080);

io.on('connection', (socket) => {
  console.log('a new client');
  // 发送string
  socket.emit('test_event_name', 'hello world, from server');
  // 发送object
  socket.emit('test_event_name', { msg: 'hello world' });
  // 发送buffer
  socket.emit('test_event_name', Buffer.from('hello world'));

  socket.on('client_event', (data, cb) => {
    console.log('client msg:', data);
    // ack确认
    cb('server ack');
  });
});
