import java from "java";
import path from "path";


export function setupClasspath() {
    const cp = path.resolve("../shared-server/lib/build/classes/java/main");
    java.classpath.push(cp);
}