package nubes.renderer;

import nubes.renderer.buffer.indexbuffer.IndexBuffer;
import nubes.renderer.buffer.vertexbuffer.VertexBuffer;
import nubes.renderer.shader.ShaderProgram;
import nubes.renderer.texture.Texture;
import org.jetbrains.annotations.NotNull;

public record RenderComponent(
        @NotNull VertexBuffer vertexBuffer,
        @NotNull IndexBuffer indexBuffer,
        @NotNull ShaderProgram shaderProgram,
        @NotNull Texture texture
) { }