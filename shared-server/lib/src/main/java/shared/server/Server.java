package shared.server;

import java.io.IOException;
import java.net.InetSocketAddress;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.Map.Entry;
import java.util.stream.Collectors;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpServer;

public class Server {

    private static final int PORT = 8001;
    private static Map<Integer, RouteHandler> routeHandlers = new HashMap<>();
    private static Map<String, Integer> routeHandlersExt = new HashMap<>();

    private static native String handle_request_external(int id, String req);

    public static void addRoute(RouteHandler routeHandler) {
        int routeIndex = Server.routeHandlers.size();
        Server.routeHandlers.put(routeIndex, routeHandler);
    }

    public static int addRoute(String path) {
        int routeIndex = Server.routeHandlersExt.size();
        Server.routeHandlersExt.put(path, routeIndex);
        System.out.println(String.format("[JAVA] registered handler path %s", path));
        return routeIndex;
    }

    public static void start(boolean lib) throws IOException {

        if (lib) {
            System.loadLibrary("shared_server_rs");
        }

        HttpServer httpServer = HttpServer.create(new InetSocketAddress("localhost", PORT), 0);

        if (lib) {
            Set<Entry<String, Integer>> routeHandlersExtList = Server.routeHandlersExt.entrySet();
            for (Entry<String, Integer> routeHandler : routeHandlersExtList) {
                System.out.println(String.format("[JAVA] registering context for %s", routeHandler.getKey()));
                httpServer.createContext(routeHandler.getKey(), exchange -> {
                    logIncomingRequest(exchange);
                    boolean matches = exchange.getRequestURI().getPath().equals(routeHandler.getKey());
                    System.out.println(matches);
                    if (matches) {
                        String request = new String(exchange.getRequestBody().readAllBytes());
                        String response = handle_request_external(routeHandler.getValue(), request);
                        exchange.sendResponseHeaders(200, response.getBytes().length);
                        exchange.getResponseBody().write(response.getBytes());
                        exchange.close();
                    }
                });
            }
        } else {
            List<RouteHandler> routeHandlersList = Server.routeHandlers.values().stream()
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
        }

        httpServer.start();

        System.out.println(String.format("[JAVA] Server started on port %d in library mode %s", PORT, lib));

    }

    private static void sendResponseWithHandler(RouteHandler routeHandler, HttpExchange exchange) throws IOException {
        String response = routeHandler.handleRequest(exchange.getRequestURI().toString());
        exchange.sendResponseHeaders(200, response.getBytes().length);
        exchange.getResponseBody().write(response.getBytes());
        exchange.close();
    }

    private static void logIncomingRequest(HttpExchange exchange) {
        System.out.println(String.format("Received request for path %s with method %s for handler path %s",
                exchange.getRequestURI().toString(), exchange.getRequestMethod(),
                exchange.getHttpContext().getPath()));
    }

    private static boolean send404IfNoMatchingPath(HttpExchange exchange, RouteHandler routeHandler)
            throws IOException {
        if (!exchange.getRequestURI().getPath().equals(routeHandler.getPath())) {
            exchange.sendResponseHeaders(404, 0);
            exchange.close();
            return true;
        }
        return false;
    }

    private static boolean send404IfNoMatchingPath(HttpExchange exchange, boolean matches)
            throws IOException {
        if (!matches) {
            exchange.sendResponseHeaders(404, 0);
            exchange.close();
            return true;
        }
        return false;
    }

}
