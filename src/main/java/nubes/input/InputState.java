package nubes.input;

import nubes.input.enums.Button;
import nubes.input.enums.Key;
import org.jetbrains.annotations.NotNull;

public interface InputState {
    boolean isKeyDown(@NotNull Key key);
    boolean isButtonDown(@NotNull Button button);
    double getCursorX();
    double getCursorY();
}
