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
import nubes.renderer.texture.OpenGLTexture;
import nubes.renderer.texture.PixelChannels;
import nubes.renderer.texture.Texture;
import nubes.transform.Rotation;
import nubes.transform.Scaling;
import nubes.transform.Transform;
import nubes.transform.Translation;
import nubes.window.GLFWWindow;
import nubes.window.Window;
import nubes.window.WindowSize;
import org.joml.AxisAngle4f;
import org.joml.Math;
import org.joml.Quaternionf;
import org.joml.Vector3f;

import java.nio.file.Path;

import static org.lwjgl.glfw.GLFW.glfwGetTime;
import static org.lwjgl.opengl.GL20.*;

public class Main {
    private static VertexBuffer vertexBuffer;
    private static IndexBuffer indexBuffer;
    private static ShaderProgram shaderProgram;
    private static Texture texture;
    private static Transform transform;

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
            texture.bind(1);

            transform.translate(new Translation((float) -Math.sin(glfwGetTime()) / 1000, (float) -Math.cos(glfwGetTime()) / 1000, 0));
            shaderProgram.setUniform("texture_slot", 1);
            shaderProgram.setUniform("model", transform.matrix());

            glDrawElements(GL_TRIANGLES, indexBuffer.count(), GL_UNSIGNED_INT, 0);
            window.swapBuffers();
        }

        vertexBuffer.delete();
        indexBuffer.delete();
        shaderProgram.delete();
        texture.delete();
    }

    public static void setup() {
        VertexLayout vertexLayout = new VertexLayout.Builder()
                .addFloats(2)
                .addFloats(2)
                .build();

        vertexBuffer = new OpenGLVertexBuffer(vertexLayout, new float[]{
                -0.5f, -0.5f, 0, 0,
                0.5f, -0.5f, 1, 0,
                0.5f, 0.5f, 1, 1,
                -0.5f, 0.5f, 0, 1
        });

        indexBuffer = new OpenGLIndexBuffer(new int[]{
                0, 1, 3,
                1, 2, 3
        });

        Path shadersPath = Path.of("src/main/resources/shaders");
        shaderProgram = new OpenGLShaderProgram.Builder()
                .addShader(ShaderType.VERTEX, shadersPath.resolve("basic_vertex_shader.vert"))
                .addShader(ShaderType.FRAGMENT, shadersPath.resolve("basic_fragment_shader.frag"))
                .build();

        Path texturesPath = Path.of("src/main/resources/textures");
        texture = new OpenGLTexture(texturesPath.resolve("cloud.png"), PixelChannels.RGBA);

        Translation translation = new Translation(0.5f, 0, 0);
        Rotation rotation = new Rotation(new Quaternionf(new AxisAngle4f(3.141592f / 3, new Vector3f(0, 0, 1))));
        Scaling scaling = new Scaling(0.5f, 2f, 1);
        transform = new Transform(translation, rotation, scaling);

        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
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