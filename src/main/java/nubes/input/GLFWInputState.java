package nubes.input;

import nubes.input.enums.Button;
import nubes.input.enums.Key;
import nubes.window.GLFWWindow;
import org.jetbrains.annotations.NotNull;
import org.lwjgl.system.MemoryStack;

import java.nio.DoubleBuffer;
import java.util.Objects;

import static org.lwjgl.glfw.GLFW.*;

public class GLFWInputState implements InputState {
    private final @NotNull GLFWWindow window;

    public GLFWInputState(@NotNull GLFWWindow window) {
        this.window = Objects.requireNonNull(window);
    }

    @Override
    public boolean isKeyDown(@NotNull Key key) {
        int res = glfwGetKey(window.id(), key.glfwKey());
        return res == GLFW_PRESS || res == GLFW_REPEAT;
    }

    @Override
    public boolean isButtonDown(@NotNull Button button) {
        int res = glfwGetMouseButton(window.id(), button.glfwButton());
        return res == GLFW_PRESS;
    }

    @Override
    public double getCursorX() {
        try (MemoryStack stack = MemoryStack.stackPush()) {
            DoubleBuffer x = stack.mallocDouble(1);
            DoubleBuffer y = stack.mallocDouble(1);
            glfwGetCursorPos(window.id(), x, y);
            return x.get();
        }
    }

    @Override
    public double getCursorY() {
        try (MemoryStack stack = MemoryStack.stackPush()) {
            DoubleBuffer x = stack.mallocDouble(1);
            DoubleBuffer y = stack.mallocDouble(1);
            glfwGetCursorPos(window.id(), x, y);
            return y.get();
        }
    }
}
