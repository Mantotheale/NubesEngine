package nubes.input.events;

import nubes.input.enums.Action;
import nubes.input.enums.Key;
import nubes.input.enums.Modifier;
import org.jetbrains.annotations.NotNull;

import java.util.List;

public record KeyInput(
        @NotNull Key key,
        @NotNull Action action,
        @NotNull List<@NotNull Modifier> modifiers) implements Input {
    public static @NotNull KeyInput fromGLFWInput(int glfwKey, int glfwAction, int glfwModifier) {
        return new KeyInput(
                Key.fromGLFWKey(glfwKey),
                Action.fromGLFWAction(glfwAction),
                Modifier.fromGLFWModifier(glfwModifier)
        );
    }
}