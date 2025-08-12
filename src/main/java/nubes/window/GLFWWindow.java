package nubes.window;

import nubes.input.*;
import org.jetbrains.annotations.NotNull;
import org.lwjgl.glfw.GLFWErrorCallback;
import org.lwjgl.opengl.GL;
import org.lwjgl.system.MemoryUtil;

import java.nio.IntBuffer;
import java.util.Objects;

import static org.lwjgl.glfw.Callbacks.glfwFreeCallbacks;
import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.glfw.GLFW.GLFW_CONTEXT_VERSION_MINOR;
import static org.lwjgl.glfw.GLFW.GLFW_OPENGL_CORE_PROFILE;
import static org.lwjgl.glfw.GLFW.GLFW_OPENGL_PROFILE;
import static org.lwjgl.glfw.GLFW.glfwCreateWindow;
import static org.lwjgl.glfw.GLFW.glfwMakeContextCurrent;
import static org.lwjgl.glfw.GLFW.glfwWindowHint;
import static org.lwjgl.opengl.GL11.glViewport;

public class GLFWWindow implements Window {
    private final long id;
    private final @NotNull String title;
    private final @NotNull IntBuffer widthBuffer;
    private final @NotNull IntBuffer heightBuffer;

    public GLFWWindow(@NotNull String title, @NotNull WindowSize size) {
        this.title = Objects.requireNonNull(title);
        widthBuffer = MemoryUtil.memAllocInt(1);
        heightBuffer = MemoryUtil.memAllocInt(1);

        GLFWErrorCallback.createPrint(System.err).set();

        if (!glfwInit()) { throw new GLFWInitializationException(); }

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

        this.id = glfwCreateWindow(size.width(), size.height(), title, 0, 0);
        if (id == 0) { throw new GLFWFailedToOpenWindowException(); }

        glfwMakeContextCurrent(id);

        GL.createCapabilities();
        glViewport(0, 0, size.width(), size.height());
    }

    @Override
    public @NotNull String title() {
        return title;
    }

    @Override
    public @NotNull WindowSize size() {
        widthBuffer.clear();
        heightBuffer.clear();
        glfwGetFramebufferSize(id, widthBuffer, heightBuffer);
        return new WindowSize(widthBuffer.get(), heightBuffer.get());
    }

    @Override
    public void enableVSync() {
        glfwSwapInterval(1);
    }

    @Override
    public void disableVSync() {
        glfwSwapInterval(0);
    }

    @Override
    public void swapBuffers() {
        glfwSwapBuffers(id);
    }

    @Override
    public void pollEvents() {
        glfwPollEvents();
    }

    @Override
    public void setKeyCallback(@NotNull InputCallback callback) {
        glfwSetKeyCallback(id, (_, key, _, action, modifier) ->
                callback.invoke(KeyInput.fromGLFWInput(key, action, modifier))
        );
    }

    @Override
    public void freeKeyCallback() {
        glfwSetKeyCallback(id, null);
    }

    @Override
    public void setMouseButtonCallback(@NotNull InputCallback callback) {
        glfwSetMouseButtonCallback(id, (_, button, action, modifier) ->
            callback.invoke(ButtonInput.fromGLFWInput(button, action, modifier))
        );
    }

    @Override
    public void freeMouseButtonCallback() {
        glfwSetMouseButtonCallback(id, null);
    }

    @Override
    public void setCursorCallback(@NotNull InputCallback callback) {
        glfwSetCursorPosCallback(id, (_, x, y) -> callback.invoke(CursorInput.fromGLFWInput(x, y)));
    }

    @Override
    public void freeCursorCallback() {
        glfwSetCursorPosCallback(id, null);
    }

    @Override
    public void setScrollCallback(@NotNull InputCallback callback) {
        glfwSetScrollCallback(id, (_, x, y) -> callback.invoke(ScrollInput.fromGLFWInput(x, y)));
    }

    @Override
    public void freeScrollCallback() {
        glfwSetScrollCallback(id, null);
    }

    @Override
    public void setResizeCallback(@NotNull InputCallback callback) {
        glfwSetFramebufferSizeCallback(id, (_, width, height) -> callback.invoke(ResizeInput.fromGLFWInput(width, height)));
    }

    @Override
    public void freeResizeCallback() {
        glfwSetFramebufferSizeCallback(id, null);
    }

    @Override
    public void setCloseCallback(@NotNull InputCallback callback) {
        glfwSetWindowCloseCallback(id, _ -> callback.invoke(CloseInput.instance()));
    }

    @Override
    public void freeCloseCallback() {
        glfwSetWindowCloseCallback(id, null);
    }

    @Override
    public void delete() {
        MemoryUtil.memFree(widthBuffer);
        MemoryUtil.memFree(heightBuffer);

        glfwFreeCallbacks(id);
        glfwDestroyWindow(id);

        glfwTerminate();
        Objects.requireNonNull(glfwSetErrorCallback(null)).free();
    }
}
