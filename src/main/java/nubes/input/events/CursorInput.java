package nubes.input.events;

import org.jetbrains.annotations.NotNull;

public record CursorInput(double x, double y) implements Input {
    public static @NotNull CursorInput fromGLFWInput(double glfwX, double glfwY) {
        return new CursorInput(glfwX, glfwY);
    }
}