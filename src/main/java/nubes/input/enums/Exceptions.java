package nubes.input.enums;

import org.jetbrains.annotations.NotNull;

class InvalidActionCodeException extends RuntimeException {
    public InvalidActionCodeException(int action) {
        super("The specified action wasn't a valid action code. The code was " + action);
    }
}

class NoCorrespondingGLFWKeyException extends RuntimeException {
    public NoCorrespondingGLFWKeyException(@NotNull Key key) {
        super("The specified key doesn't have a corresponding key in GLFW. The key was " + key);
    }
}

class NoCorrespondingGLFWButtonException extends RuntimeException {
    public NoCorrespondingGLFWButtonException(@NotNull Button button) {
        super("The specified button doesn't have a corresponding button in GLFW. The button was " + button);
    }
}