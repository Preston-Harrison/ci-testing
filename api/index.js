const express = require('express');
const app = express();

app.get('/hello', (req, res) => {
    res.send("Hi There!")
});

app.listen(+(process.env.PORT), () => {
    console.log(`App listening on port ${process.env.PORT}`)
});