package nubes.window;

class InvalidWindowSizeException extends RuntimeException {
    public InvalidWindowSizeException(int width, int height) {
        super("Width and height of the window should be positive. They were (" + width + ", " + height + ").");
    }
}

class GLFWInitializationException extends RuntimeException {
    public GLFWInitializationException() {
        super("Couldn't initialize GLFW");
    }
}

class GLFWFailedToOpenWindowException extends RuntimeException {
    public GLFWFailedToOpenWindowException() {
        super("Failed to open a window");
    }
}