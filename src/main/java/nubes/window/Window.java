package nubes.window;

import nubes.input.events.InputCallback;
import org.jetbrains.annotations.NotNull;

public interface Window {
    @NotNull String title();
    @NotNull WindowSize size();

    void enableVSync();
    void disableVSync();
    void swapBuffers();

    void pollEvents();
    void setKeyCallback(@NotNull InputCallback callback);
    void freeKeyCallback();
    void setMouseButtonCallback(@NotNull InputCallback callback);
    void freeMouseButtonCallback();
    void setCursorCallback(@NotNull InputCallback callback);
    void freeCursorCallback();
    void setScrollCallback(@NotNull InputCallback callback);
    void freeScrollCallback();
    void setResizeCallback(@NotNull InputCallback callback);
    void freeResizeCallback();
    void setCloseCallback(@NotNull InputCallback callback);
    void freeCloseCallback();

    void delete();
}
