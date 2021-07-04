const express = require('express');
const app = express();

app.use(express.static('static'));

if (process.argv.slice(2).includes('--debug')) {
    console.log('[*] Running in debug mode');
    app.get('/EXIT', (req, res) => {
        res.send('ok');
        process.exit(0)
    })
}

app.listen(3000, () => {
  console.log('[*] Server listening on port 3000');
});
