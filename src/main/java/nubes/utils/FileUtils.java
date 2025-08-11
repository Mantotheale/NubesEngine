package nubes.utils;

import org.jetbrains.annotations.NotNull;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Objects;

public final class FileUtils {
    private FileUtils() { }

    public static @NotNull String fileToString(@NotNull Path filePath) {
        try {
            return Files.readString(Objects.requireNonNull(filePath));
        } catch (IOException e) {
            throw new FailToOpenFileException(filePath);
        }
    }

    public static @NotNull String fileToString(@NotNull String filePathString) {
        return fileToString(Path.of(Objects.requireNonNull(filePathString)));
    }
}