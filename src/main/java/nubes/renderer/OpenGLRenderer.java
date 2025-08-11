package nubes.renderer;

import nubes.renderer.buffer.indexbuffer.IndexBuffer;
import nubes.renderer.buffer.vertexbuffer.VertexBuffer;
import nubes.renderer.shader.ShaderProgram;
import nubes.renderer.texture.Texture;
import nubes.transform.Transform;
import org.jetbrains.annotations.NotNull;

import java.util.ArrayList;
import java.util.List;

import static org.lwjgl.opengl.GL11.*;

public class OpenGLRenderer implements Renderer {
    private final @NotNull List<VertexBuffer> vertexBuffers = new ArrayList<>();
    private final @NotNull List<IndexBuffer> indexBuffers = new ArrayList<>();
    private final @NotNull List<ShaderProgram> shaderPrograms = new ArrayList<>();
    private final @NotNull List<Texture> textures = new ArrayList<>();
    private final @NotNull List<Transform> transforms = new ArrayList<>();

    @Override
    public void submit(@NotNull VertexBuffer vertexBuffer, @NotNull IndexBuffer indexBuffer, @NotNull ShaderProgram shaderProgram, @NotNull Texture texture, @NotNull Transform transform) {
        vertexBuffers.add(vertexBuffer);
        indexBuffers.add(indexBuffer);
        shaderPrograms.add(shaderProgram);
        textures.add(texture);
        transforms.add(transform);
    }

    @Override
    public void draw() {
        for (int i = 0; i < vertexBuffers.size(); i++) {
            vertexBuffers.get(i).bind();
            indexBuffers.get(i).bind();
            shaderPrograms.get(i).bind();
            textures.get(i).bind(1);

            shaderPrograms.get(i).setUniform("texture_slot", 1);
            shaderPrograms.get(i).setUniform("model", transforms.get(i).matrix());

            glDrawElements(GL_TRIANGLES, indexBuffers.get(i).count(), GL_UNSIGNED_INT, 0);
        }
    }

    @Override
    public void flush() {
        vertexBuffers.clear();
        indexBuffers.clear();
        shaderPrograms.clear();
        textures.clear();
        transforms.clear();
    }

    @Override
    public void setClearColor(float r, float g, float b, float a) {
        glClearColor(r, g, b, a);
    }

    @Override
    public void clearColor() {
        glClear(GL_COLOR_BUFFER_BIT);
    }

    @Override
    public void clearDepth() {
        glClear(GL_DEPTH_BUFFER_BIT);
    }

    @Override
    public void enableDepthTesting() {
        glEnable(GL_DEPTH_TEST);
    }

    @Override
    public void disableDepthTesting() {
        glDisable(GL_DEPTH_TEST);
    }

    @Override
    public void enableBlending() {
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    }

    @Override
    public void disableBlending() {
        glDisable(GL_BLEND);
    }
}
