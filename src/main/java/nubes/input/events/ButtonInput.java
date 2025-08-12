package nubes.input.events;

import nubes.input.enums.Action;
import nubes.input.enums.Button;
import nubes.input.enums.Modifier;
import org.jetbrains.annotations.NotNull;

import java.util.List;

public record ButtonInput(
        @NotNull Button button,
        @NotNull Action action,
        @NotNull List<@NotNull Modifier> modifiers) implements Input {
    public static @NotNull ButtonInput fromGLFWInput(int glfwButton, int glfwAction, int glfwModifier) {
        return new ButtonInput(
                Button.fromGLFWButton(glfwButton),
                Action.fromGLFWAction(glfwAction),
                Modifier.fromGLFWModifier(glfwModifier)
        );
    }
}