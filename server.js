const roadRunner = require("./index.node");
const { callbackify } = require("util");

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

function helloWorld(success, req) {
  return async () => {
    console.log(req);
    const res = await axios.get(req.body);
    return res.data;
  };
}

function helloWorldPost(success, req) {
  if (success == null) {
    var body = getBuffer(req);
    return "hello world post " + body;
  }
}

class App {
  constructor() {
    this.router = roadRunner.createRouter();
    this.server = roadRunner.createServer(this.router);
  }

  get = function (path, handler) {
    roadRunner.addRoute.call(this.router, "GET", path, handler);
  };
  post = function (path, handler) {
    roadRunner.addRoute.call(this.router, "POST", path, handler);
  };
  listen = function (port) {
    roadRunner.startServer.call(this.server, port);
  };

  addFunction = function (handler) {
    roadRunner.addHandler.call(this.app, handler);
  };
}

const app = new App();

app.get("/", helloWorld);
// app.addRoute("/", helloWorldPost);
// app.listen(3000);

// const functions = new functionsHandler();
// console.log(functions);
// var keys = Object.keys(functions);
// console.log(keys);
// startServer(functions);
// class functionsHandler {
//   constructor() {
//     return functionsHandlerNew("GET", "/users", (success, req) => {
//       let name = req.params.params;
//       if (name == "ramyak") {
//         return `hello ${name}`;
//       }
//     });
//   }
// }
