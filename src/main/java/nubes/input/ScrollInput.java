package nubes.input;

import org.jetbrains.annotations.NotNull;

public record ScrollInput(double x, double y) implements Input {
    public static @NotNull ScrollInput fromGLFWInput(double glfwX, double glfwY) {
        return new ScrollInput(glfwX, glfwY);
    }
}