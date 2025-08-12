package nubes;

import nubes.window.GLFWWindow;
import nubes.window.Window;
import nubes.window.WindowSize;
import org.jetbrains.annotations.NotNull;

import java.util.Objects;

import static org.lwjgl.glfw.GLFW.glfwGetTime;
import static org.lwjgl.glfw.GLFW.glfwPollEvents;

public abstract class Nubes {
    private final static double ONE_SEC_TIME = 1;

    private final @NotNull Window window;
    private final double UPDATE_TIME;
    private boolean shouldClose;

    public Nubes(@NotNull String windowTitle, @NotNull WindowSize size, int fps) {
        this.window = new GLFWWindow(Objects.requireNonNull(windowTitle), size);
        window.enableVSync();

        window.setCloseCallback(this::signalClose);

        this.UPDATE_TIME = (double) 1 / fps;
        this.shouldClose = false;
    }

    public final void run() {
        double currentTime = glfwGetTime();
        double nextUpdateTime = currentTime + UPDATE_TIME;
        double nextOneSecTime = currentTime + ONE_SEC_TIME;

        while (!shouldClose) {
            glfwPollEvents();

            currentTime = glfwGetTime();
            while (currentTime >= nextUpdateTime) {
                update();
                nextUpdateTime += UPDATE_TIME;
            }

            render();
            window.swapBuffers();

            while (currentTime >= nextOneSecTime) {
                oneSecUpdate();
                nextOneSecTime += ONE_SEC_TIME;
            }
        }

        terminate();
    }

    protected abstract void update();

    protected abstract void oneSecUpdate();

    protected abstract void render();

    protected abstract void terminate();

    public void signalClose() {
        shouldClose = true;
    }
}
