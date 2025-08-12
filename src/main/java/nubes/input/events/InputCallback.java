package nubes.input.events;

import org.jetbrains.annotations.NotNull;

@FunctionalInterface
public interface InputCallback {
    void invoke(@NotNull Input input);
}
