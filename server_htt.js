const express = require("express");
const app = express();
const port = 3000;

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

app.get("/hello/:name", async (req, res) => {
  let result = `hello ${req.params.name}`;
  await sleep(5000);
  res.send(result);
});

app.get("/", (req, res) => {
  res.send("hello world");
});

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
