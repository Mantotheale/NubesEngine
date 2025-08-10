package nubes.input.enums;

import org.jetbrains.annotations.NotNull;

import static org.lwjgl.glfw.GLFW.GLFW_MOUSE_BUTTON_LEFT;
import static org.lwjgl.glfw.GLFW.GLFW_MOUSE_BUTTON_RIGHT;

public enum Button {
    LEFT,
    RIGHT,
    NO_BUTTON;

    public static @NotNull Button fromGLFWButton(int glfwButton) {
        return switch (glfwButton) {
            case GLFW_MOUSE_BUTTON_LEFT -> LEFT;
            case GLFW_MOUSE_BUTTON_RIGHT -> RIGHT;
            default -> NO_BUTTON;
        };
    }
}
