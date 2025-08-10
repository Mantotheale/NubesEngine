package nubes.window;

import nubes.input.*;
import org.jetbrains.annotations.NotNull;

public interface Window {
    @NotNull String title();
    @NotNull WindowSize size();

    void enableVSync();
    void disableVSync();
    void swapBuffers();

    void pollEvents();
    void setKeyCallback(@NotNull KeyCallback keyCallback);
    void freeKeyCallback();
    void setMouseButtonCallback(@NotNull ButtonCallback buttonCallback);
    void freeMouseButtonCallback();
    void setCursorCallback(@NotNull CursorCallback cursorCallback);
    void freeCursorCallback();
    void setScrollCallback(@NotNull ScrollCallback scrollCallback);
    void freeScrollCallback();
    void setResizeCallback(@NotNull WindowResizeCallback resizeCallback);
    void freeResizeCallback();
    void setCloseCallback(@NotNull CloseWindowCallback closeCallback);
    void freeCloseCallback();

    void delete();
}
