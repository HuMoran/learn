const io = require('socket.io-client');

const socket = io.connect('http://localhost:8080');
socket.on('test_event_name', (data) => {
  console.log('server msg:', data);
});

setTimeout(() => {
  console.log('start setTimeout');
  socket.emit('client_event', 'msg from setTimeout', (data) =>{
    console.log('ack data:', data);
  });
}, 3000);

socket.on('error', (error) => {
  console.log('error:', error);
});

socket.on('connect_error', (error) => {
  console.log('connect error:', error);
});

socket.on('connect_timeout', (error) => {
  console.log('timeout error:', error);
});
