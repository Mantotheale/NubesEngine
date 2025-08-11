package nubes;

import nubes.input.enums.Key;
import nubes.renderer.buffer.OpenGLVertexBuffer;
import nubes.renderer.buffer.VertexBuffer;
import nubes.renderer.buffer.VertexLayout;
import nubes.window.GLFWWindow;
import nubes.window.Window;
import nubes.window.WindowSize;

import static org.lwjgl.opengl.GL20.*;

public class Main {
    private static VertexBuffer vertexBuffer;
    private static int shaderProgram;

    public static void main(String[] args) {
        Window window = new GLFWWindow("Hello World!", new WindowSize(1280, 720));

        MutableBool shouldClose = new MutableBool(false);
        window.setCloseCallback(() -> shouldClose.set(true));
        window.setKeyCallback((key, _, _) -> {
            if (key.equals(Key.ESC)) {
                shouldClose.set(true);
            }
        });
        window.enableVSync();

        setup();
        while (!shouldClose.get()) {
            window.pollEvents();

            glClearColor(0.2f, 0.3f, 0.3f, 1.0f);
            glClear(GL_COLOR_BUFFER_BIT);

            vertexBuffer.bind();
            glUseProgram(shaderProgram);

            glDrawArrays(GL_TRIANGLES, 0, 3);
            window.swapBuffers();
        }
    }

    public static void setup() {
        VertexLayout vertexLayout = new VertexLayout.Builder()
                .addFloats(2)
                .build();

        vertexBuffer = new OpenGLVertexBuffer(vertexLayout, new float[] {
                -0.5f, -0.5f,
                0.5f, -0.5f,
                0, 0.5f
        });

        String vertexShaderSource = """
                #version 330 core
                
                layout (location = 0) in vec2 aPos;
                
                void main()
                {
                   gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
                }\0""";

        String fragmentShaderSource = """
                #version 330 core
                
                out vec4 FragColor;
                
                void main()
                {
                   FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
                }
                \0""";

        int vertexShader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertexShader, vertexShaderSource);
        glCompileShader(vertexShader);

        if (glGetShaderi(vertexShader, GL_COMPILE_STATUS) == GL_FALSE)
            throw new IllegalStateException("Couldn't compile vertex shader");

        int fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragmentShader, fragmentShaderSource);
        glCompileShader(fragmentShader);

        if (glGetShaderi(fragmentShader, GL_COMPILE_STATUS) == GL_FALSE)
            throw new IllegalStateException("Couldn't compile fragment shader");

        shaderProgram = glCreateProgram();
        glAttachShader(shaderProgram, vertexShader);
        glAttachShader(shaderProgram, fragmentShader);
        glLinkProgram(shaderProgram);

        if(glGetProgrami(shaderProgram, GL_LINK_STATUS) == GL_FALSE)
            throw new IllegalStateException("Couldn't link shader");

        glDeleteShader(vertexShader);
        glDeleteShader(fragmentShader);
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