/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package shared.server;

import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.Map;

import org.junit.jupiter.api.Test;

class ServerTest {
    @Test
    void shouldAddRoute()
            throws NoSuchFieldException, SecurityException, IllegalArgumentException, IllegalAccessException {
        Server classUnderTest = new Server();
        classUnderTest.addRoute(
                new RouteHandler("/", (request) -> request));
        var field = classUnderTest.getClass().getDeclaredField("routeHandlers");
        field.setAccessible(true);
        Map<Integer, RouteHandler> routeHandlers = (Map<Integer, RouteHandler>) field.get(classUnderTest);
        assertTrue(
                routeHandlers.values().stream().anyMatch(routeHandler -> routeHandler.getPath().equals("/")));
    }
}
