package nubes.renderer.buffer;

import org.jetbrains.annotations.NotNull;

public interface VertexBuffer {
    void bind();
    void unbind();
    @NotNull VertexLayout layout();
    void delete();
}
