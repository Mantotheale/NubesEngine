package nubes;

import nubes.window.WindowSize;
import snake.App;

public class Main {
    public static void main(String[] args) {
        App app = new App("Hello World!", new WindowSize(1280, 720), 60);
        app.run();
    }
}