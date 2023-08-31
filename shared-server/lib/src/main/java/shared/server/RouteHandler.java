package shared.server;

import java.util.function.Function;

public class RouteHandler {

    private String path;
    private Function<String, String> handler;

    public RouteHandler(String path, Function<String, String> handler) {
        this.path = path;
        this.handler = handler;
    }

    public String getPath() {
        return this.path;
    }

    public String handleRequest(String request) {
        return this.handler.apply(request);
    }

}
