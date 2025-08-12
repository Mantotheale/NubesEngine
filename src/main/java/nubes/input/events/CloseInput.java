package nubes.input.events;

import org.jetbrains.annotations.NotNull;

public record CloseInput() implements Input {
    private final static @NotNull CloseInput instance = new CloseInput();

    public static @NotNull CloseInput instance() {
        return instance;
    }
}