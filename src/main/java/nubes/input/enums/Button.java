package nubes.input.enums;

import org.jetbrains.annotations.NotNull;

import static org.lwjgl.glfw.GLFW.GLFW_MOUSE_BUTTON_LEFT;
import static org.lwjgl.glfw.GLFW.GLFW_MOUSE_BUTTON_RIGHT;

public enum Button {
    LEFT {
        @Override
        public int glfwButton() {
            return GLFW_MOUSE_BUTTON_LEFT;
        }
    },
    RIGHT {
        @Override
        public int glfwButton() {
            return GLFW_MOUSE_BUTTON_RIGHT;
        }
    },
    NO_BUTTON {
        @Override
        public int glfwButton() {
            throw new NoCorrespondingGLFWButtonException(this);
        }
    };

    public abstract int glfwButton();

    public static @NotNull Button fromGLFWButton(int glfwButton) {
        return switch (glfwButton) {
            case GLFW_MOUSE_BUTTON_LEFT -> LEFT;
            case GLFW_MOUSE_BUTTON_RIGHT -> RIGHT;
            default -> NO_BUTTON;
        };
    }
}
