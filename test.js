const http = require('http');
const body = JSON.stringify({ mode: 'stub' });
const options = {
  hostname: 'localhost',
  port: 4317,
  path: '/orchestrator/mode',
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Content-Length': Buffer.byteLength(body)
  }
};
const req = http.request(options, (res) => {
  let data = '';
  res.on('data', (chk) => data += chk);
  res.on('end', () => console.log('Status:', res.statusCode, 'Body:', data));
});
req.on('error', (e) => console.error(e));
req.write(body);
req.end();
