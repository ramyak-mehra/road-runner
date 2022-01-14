const express = require("express");
const app = express();
const port = 3000;



app.get("/", (req, res) => {
  res.send("hello world");
});

app.post("/" , (req, res) => {
  console.log(req.body);
  res.send("hello world post " + req.body);
})

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
