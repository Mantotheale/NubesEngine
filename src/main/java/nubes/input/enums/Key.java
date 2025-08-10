package nubes.input.enums;

import org.jetbrains.annotations.NotNull;

import static org.lwjgl.glfw.GLFW.*;

public enum Key {
    W,
    A,
    S,
    D,
    ESC,
    NO_KEY;

    public static @NotNull Key fromGLFWKey(int glfwKey) {
        return switch (glfwKey) {
            case GLFW_KEY_W -> W;
            case GLFW_KEY_A -> A;
            case GLFW_KEY_S -> S;
            case GLFW_KEY_D -> D;
            case GLFW_KEY_ESCAPE -> ESC;
            default -> NO_KEY;
        };
    }
}
