package shared.server;

import java.io.IOException;
import java.net.InetSocketAddress;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpServer;

public class Server {

    private static final int PORT = 8001;
    private Map<Integer, RouteHandler> routeHandlers = new HashMap<>();

    public void addRoute(RouteHandler routeHandler) {
        int routeIndex = this.routeHandlers.size();
        this.routeHandlers.put(routeIndex, routeHandler);
    }

    public void start() throws IOException {

        HttpServer httpServer = HttpServer.create(new InetSocketAddress("localhost", PORT), 0);

        List<RouteHandler> routeHandlersList = this.routeHandlers.values().stream()
                .collect(Collectors.toList());
        for (RouteHandler routeHandler : routeHandlersList) {
            httpServer.createContext(routeHandler.getPath(), exchange -> {
                logIncomingRequest(exchange);

                if (send404IfNoMatchingPath(exchange, routeHandler)) {
                    return;
                }

                sendResponseWithHandler(routeHandler, exchange);
            });
        }

        httpServer.start();

        System.out.println(String.format("Server started on port %d", PORT));

    }

    private void sendResponseWithHandler(RouteHandler routeHandler, HttpExchange exchange) throws IOException {
        String response = routeHandler.handleRequest(exchange.getRequestURI().toString());
        exchange.sendResponseHeaders(200, response.getBytes().length);
        exchange.getResponseBody().write(response.getBytes());
        exchange.close();
    }

    private void logIncomingRequest(HttpExchange exchange) {
        System.out.println(String.format("Received request for path %s with method %s for handler path %s",
                exchange.getRequestURI().toString(), exchange.getRequestMethod(),
                exchange.getHttpContext().getPath()));
    }

    private boolean send404IfNoMatchingPath(HttpExchange exchange, RouteHandler routeHandler) throws IOException {
        if (!exchange.getRequestURI().getPath().equals(routeHandler.getPath())) {
            exchange.sendResponseHeaders(404, 0);
            exchange.close();
            return true;
        }
        return false;
    }

}
