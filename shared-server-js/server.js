import java from "java";
export function addRoute(path, fn) {
    const fnProxy = java.newProxy("java.util.function.Function", {
        apply: fn
    });

    const routeHandler = java.newInstanceSync("shared.server.RouteHandler", path, fnProxy);
    java.callStaticMethodSync("shared.server.Server", "addRoute", routeHandler);
}

export function start() {
    java.callStaticMethodSync("shared.server.Server", "start", false);
}