package nubes.window;

public record WindowSize(
        int width,
        int height) {
    public WindowSize {
        if (width <= 0 || height <= 0) { throw new InvalidWindowSizeException(width, height); }
    }
}
