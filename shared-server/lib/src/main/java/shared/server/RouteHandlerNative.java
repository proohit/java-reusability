package shared.server;

import java.util.function.Function;

public class RouteHandlerNative implements Function<String, String> {
    public String path;

    public RouteHandlerNative(String path) {
        this.path = path;
    }

    public native String apply(String arg0);
}
