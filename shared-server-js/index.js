import { setupClasspath } from "./jvm.js";
import { addRoute, start } from "./server.js";

setupClasspath();

addRoute("/test1", () => "test1");
addRoute("/test2", () => "test2");

start();