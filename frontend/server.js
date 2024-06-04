import express from 'express';
import http from 'http';
const app = express();
const port = process.env.PORT || 4447;

app.use('/', express.static('public'));
app.use('/pkg', express.static('public/pkg'));

http.createServer(app).listen(port, () => {
    console.log(`Server is running on https://localhost:${port}`);
});