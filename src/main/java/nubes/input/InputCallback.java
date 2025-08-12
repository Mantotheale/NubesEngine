package nubes.input;

import org.jetbrains.annotations.NotNull;

@FunctionalInterface
public interface InputCallback {
    void invoke(@NotNull Input input);
}
