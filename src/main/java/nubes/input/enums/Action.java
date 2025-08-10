package nubes.input.enums;

import org.jetbrains.annotations.NotNull;

import static org.lwjgl.glfw.GLFW.*;

public enum Action {
    PRESS,
    RELEASE;

    public static @NotNull Action fromGLFWAction(int glfwAction) {
        return switch (glfwAction) {
            case GLFW_PRESS, GLFW_REPEAT -> PRESS;
            case GLFW_RELEASE -> RELEASE;
            default -> throw new InvalidActionCodeException(glfwAction);
        };
    }
}
