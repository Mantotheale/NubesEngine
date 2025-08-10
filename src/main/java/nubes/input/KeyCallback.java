package nubes.input;

import nubes.input.enums.Action;
import nubes.input.enums.Key;
import nubes.input.enums.Modifier;
import org.jetbrains.annotations.NotNull;

import java.util.List;

@FunctionalInterface
public interface KeyCallback {
    void callback(@NotNull Key key, @NotNull Action action, @NotNull List<@NotNull Modifier> modifiers);
}
