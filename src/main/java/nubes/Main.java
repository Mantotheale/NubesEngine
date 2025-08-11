package nubes;

import nubes.input.enums.Key;
import nubes.renderer.buffer.indexbuffer.IndexBuffer;
import nubes.renderer.buffer.indexbuffer.OpenGLIndexBuffer;
import nubes.renderer.buffer.vertexbuffer.OpenGLVertexBuffer;
import nubes.renderer.buffer.vertexbuffer.VertexBuffer;
import nubes.renderer.buffer.vertexbuffer.VertexLayout;
import nubes.renderer.shader.OpenGLShaderProgram;
import nubes.renderer.shader.ShaderProgram;
import nubes.renderer.shader.ShaderType;
import nubes.window.GLFWWindow;
import nubes.window.Window;
import nubes.window.WindowSize;

import java.nio.file.Path;

import static org.lwjgl.opengl.GL20.*;

public class Main {
    private static VertexBuffer vertexBuffer;
    private static IndexBuffer indexBuffer;
    private static ShaderProgram shaderProgram;

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
            indexBuffer.bind();
            shaderProgram.bind();

            glDrawElements(GL_TRIANGLES, indexBuffer.count(), GL_UNSIGNED_INT, 0);
            window.swapBuffers();
        }

        vertexBuffer.delete();
        indexBuffer.delete();
        shaderProgram.delete();
    }

    public static void setup() {
        VertexLayout vertexLayout = new VertexLayout.Builder()
                .addFloats(2)
                .build();

        vertexBuffer = new OpenGLVertexBuffer(vertexLayout, new float[] {
                -0.5f, -0.5f,
                0.5f, -0.5f,
                0.5f, 0.5f,
                -0.5f, 0.5f
        });

        indexBuffer = new OpenGLIndexBuffer(new int[] {
                0, 1, 3,
                1, 2, 3
        });

        Path shadersPath = Path.of("src/main/resources/shaders");
        shaderProgram = new OpenGLShaderProgram.Builder()
                .addShader(ShaderType.VERTEX, shadersPath.resolve("basic_vertex_shader.vert"))
                .addShader(ShaderType.FRAGMENT, shadersPath.resolve("basic_fragment_shader.frag"))
                .build();
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