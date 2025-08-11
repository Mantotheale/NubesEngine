package nubes.utils;

import org.jetbrains.annotations.NotNull;

import java.nio.file.Path;

class FailToOpenFileException extends RuntimeException {
    public FailToOpenFileException(@NotNull Path path) {
        super("Couldn't open file " + path);
    }
}