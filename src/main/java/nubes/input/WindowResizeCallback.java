package nubes.input;

@FunctionalInterface
public interface WindowResizeCallback {
    void callback(int width, int height);
}