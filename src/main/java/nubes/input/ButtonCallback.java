package nubes.input;

import nubes.input.enums.Action;
import nubes.input.enums.Button;
import nubes.input.enums.Modifier;
import org.jetbrains.annotations.NotNull;

import java.util.List;

@FunctionalInterface
public interface ButtonCallback {
    void callback(@NotNull Button button, @NotNull Action action, @NotNull List<@NotNull Modifier> modifiers);
}
