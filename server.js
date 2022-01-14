const roadRunner = require("./index.node");
const { promisify } = require("util");

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

function ab2str(buf) {
  return String.fromCharCode.apply(null, new Uint16Array(buf));
}

function getBuffer(buf){
  return buf.body;
}

function helloWorld(success, req) {
  return "hello world";
}
function helloWorldPost(success, req) {
  if (success == null) {
    var body = getBuffer(req);
    return "hello world post " + body;
  }
}

class Router {
  constructor() {
    this.router = roadRunner.createRouter();
  }
  addRoute = function (type, path, handler) {
    if (type === "GET") {
      roadRunner.addGetRoute.call(this.router, path, handler);
    } else {
      roadRunner.addPostRoute.call(this.router, path, handler);
    }
  };
}

class Server{
  constructor(router){
    this.server = roadRunner.createServer(router.router);
  }
  listen = function(port){
    roadRunner.startServer.call(this.server, port);
  }
}




const router = new Router();
router.addRoute("GET", "/", helloWorld);
const server = new Server(router);
router.addRoute("POST", "/", helloWorldPost);
server.listen(3000);

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
