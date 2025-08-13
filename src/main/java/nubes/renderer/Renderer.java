package nubes.renderer;

import nubes.renderer.buffer.indexbuffer.IndexBuffer;
import nubes.renderer.buffer.vertexbuffer.VertexBuffer;
import nubes.renderer.shader.ShaderProgram;
import nubes.renderer.texture.Texture;
import nubes.transform.Transform;
import org.jetbrains.annotations.NotNull;

public interface Renderer {
    void submit(
            @NotNull VertexBuffer vertexBuffer,
            @NotNull IndexBuffer indexBuffer,
            @NotNull ShaderProgram shaderProgram,
            @NotNull Texture texture,
            @NotNull Transform transform);
    void submit(
            @NotNull RenderComponent renderComponent,
            @NotNull Transform transform);
    void draw();
    void flush();
    void setClearColor(float r, float g, float b, float a);
    void clearColor();
    void clearDepth();
    void enableDepthTesting();
    void disableDepthTesting();
    void enableBlending();
    void disableBlending();
}