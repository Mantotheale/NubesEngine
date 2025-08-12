package nubes;

import nubes.input.CloseInput;
import nubes.input.Input;
import nubes.input.KeyInput;
import nubes.input.enums.Action;
import nubes.input.enums.Key;
import nubes.renderer.OpenGLRenderer;
import nubes.renderer.Renderer;
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
import nubes.window.WindowSize;
import org.jetbrains.annotations.NotNull;
import org.joml.AxisAngle4f;
import org.joml.Math;
import org.joml.Quaternionf;
import org.joml.Vector3f;

import java.nio.file.Path;
import java.util.Objects;

import static org.lwjgl.glfw.GLFW.glfwGetTime;

public class App extends Nubes {
    private static Renderer renderer;
    private static VertexBuffer vertexBuffer;
    private static IndexBuffer indexBuffer;
    private static ShaderProgram shaderProgram;
    private static Texture texture;
    private static Transform transform;

    public App(@NotNull String windowTitle, @NotNull WindowSize size, int fps) {
        super(Objects.requireNonNull(windowTitle), Objects.requireNonNull(size), fps);

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

        renderer = new OpenGLRenderer();
        renderer.enableBlending();
    }

    @Override
    protected void update() {

    }

    @Override
    protected void oneSecUpdate() {

    }

    @Override
    protected void onInput(@NotNull Input input) {
        switch (input) {
            case CloseInput _ -> signalClose();
            case KeyInput(Key key, Action action, _) when key.equals(Key.ESC) && action.equals(Action.PRESS) ->
                    signalClose();
            default -> {}
        }
    }

    @Override
    protected void render() {
        renderer.setClearColor(0.2f, 0.3f, 0.3f, 1.0f);
        renderer.clearColor();

        transform.translate(new Translation((float) -Math.sin(glfwGetTime()) / 1000, (float) -Math.cos(glfwGetTime()) / 1000, 0));
        renderer.submit(vertexBuffer, indexBuffer, shaderProgram, texture, transform);

        renderer.draw();
        renderer.flush();
    }

    @Override
    protected void terminate() {
        vertexBuffer.delete();
        indexBuffer.delete();
        shaderProgram.delete();
        texture.delete();
    }
}
