const { startServer, functionsHandlerNew, get } = require("./index.node");

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

class functionsHandler {
  constructor() {
    return functionsHandlerNew("GET", "/users",  (success, req) => {
      let name = req.params.params;
      if (name == "ramyak") {

        return `hello ${name}`;
      }
    });
  }
}
const functions = new functionsHandler();
// console.log(functions);
// var keys = Object.keys(functions);
// console.log(keys);
startServer(functions);
