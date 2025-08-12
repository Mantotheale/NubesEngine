package nubes.input.enums;

import org.jetbrains.annotations.NotNull;

import static org.lwjgl.glfw.GLFW.*;

public enum Key {
    W {
        @Override
        public int glfwKey() {
            return GLFW_KEY_W;
        }
    },
    A {
        @Override
        public int glfwKey() {
            return GLFW_KEY_A;
        }
    },
    S {
        @Override
        public int glfwKey() {
            return GLFW_KEY_S;
        }
    },
    D {
        @Override
        public int glfwKey() {
            return GLFW_KEY_D;
        }
    },
    ESC {
        @Override
        public int glfwKey() {
            return GLFW_KEY_ESCAPE;
        }
    },
    NO_KEY {
        @Override
        public int glfwKey() {
            throw new NoCorrespondingGLFWKeyException(this);
        }
    };

    public abstract int glfwKey();

    public static @NotNull Key fromGLFWKey(int glfwKey) {
        return switch (glfwKey) {
            case GLFW_KEY_W -> W;
            case GLFW_KEY_A -> A;
            case GLFW_KEY_S -> S;
            case GLFW_KEY_D -> D;
            case GLFW_KEY_ESCAPE -> ESC;
            default -> NO_KEY;
        };
    }
}
