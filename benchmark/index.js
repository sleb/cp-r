const Benchmark = require("benchmark");
const rfdc = require("rfdc")();
const data = require("./data.json");
const cp = require("..");

const suite = new Benchmark.Suite();

suite
  .add("RegExp#test", function () {
    /o/.test("Hello World!");
  })
  .add("String#indexOf", function () {
    "Hello World!".indexOf("o") > -1;
  })
  .add("rfdc", function () {
    rfdc(data);
  })
  .add("cp", function () {
    cp.hello();
  })
  // add listeners
  .on("cycle", function (event) {
    console.log(String(event.target));
  })
  .on("complete", function () {
    console.log("Fastest is " + this.filter("fastest").map("name"));
  })
  // run async
  .run({ async: true });
