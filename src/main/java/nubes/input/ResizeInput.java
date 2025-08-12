package nubes.input;

import org.jetbrains.annotations.NotNull;

public record ResizeInput(int width, int height) implements Input {
    public static @NotNull ResizeInput fromGLFWInput(int glfwWidth, int glfwHeight) {
        return new ResizeInput(glfwWidth, glfwHeight);
    }
}