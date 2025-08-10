package nubes;

import nubes.input.enums.Key;
import nubes.window.GLFWWindow;
import nubes.window.Window;
import nubes.window.WindowSize;

public class Main {
    public static void main(String[] args) {
        Window window = new GLFWWindow("Hello World!", new WindowSize(1280, 720));

        MutableBool shouldClose = new MutableBool(false);
        window.setCloseCallback(() -> shouldClose.set(true));
        window.setKeyCallback((key, _, _) -> {
            if (key.equals(Key.ESC)) {
                shouldClose.set(true);
            }
        });

        while (!shouldClose.get()) {
            window.pollEvents();
        }
    }

    private static class MutableBool {
        private boolean value;

        public MutableBool(boolean value) {
            this.value = value;
        }

        public boolean get() {
            return value;
        }

        public void set(boolean value) {
            this.value = value;
        }
    }
}