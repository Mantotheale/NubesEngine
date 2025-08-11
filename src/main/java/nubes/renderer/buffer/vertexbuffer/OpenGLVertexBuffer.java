package nubes.renderer.buffer.vertexbuffer;

import org.jetbrains.annotations.NotNull;

import java.util.Objects;

import static org.lwjgl.opengl.GL15.*;
import static org.lwjgl.opengl.GL15.GL_ARRAY_BUFFER;
import static org.lwjgl.opengl.GL15.GL_STATIC_DRAW;
import static org.lwjgl.opengl.GL15.glBindBuffer;
import static org.lwjgl.opengl.GL15.glBufferData;
import static org.lwjgl.opengl.GL20.glEnableVertexAttribArray;
import static org.lwjgl.opengl.GL20.glVertexAttribPointer;
import static org.lwjgl.opengl.GL30.*;

public class OpenGLVertexBuffer implements VertexBuffer {
    private final int vaoId;
    private final int vboId;
    private final @NotNull VertexLayout layout;
    private boolean isDeleted;

    public OpenGLVertexBuffer(@NotNull VertexLayout layout, float @NotNull [] data) {
        this.vaoId = glGenVertexArrays();
        this.vboId = glGenBuffers();
        this.layout = Objects.requireNonNull(layout);
        this.isDeleted = false;

        glBindVertexArray(vaoId);
        glBindBuffer(GL_ARRAY_BUFFER, vboId);
        glBufferData(GL_ARRAY_BUFFER, Objects.requireNonNull(data), GL_STATIC_DRAW);

        for (LayoutElement element: layout) {
            glVertexAttribPointer(
                    element.index(),
                    element.primitiveCount(),
                    element.primitiveType().openGLType(),
                    element.isNormalized(),
                    layout.size(),
                    element.offset()
            );
            glEnableVertexAttribArray(element.index());
        }

        glBindVertexArray(0);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
    }

    @Override
    public void bind() {
        if (isDeleted) { throw new VertexBufferIsDeletedException(this); }
        glBindVertexArray(vaoId);
    }

    @Override
    public void unbind() {
        if (isDeleted) { throw new VertexBufferIsDeletedException(this); }
        glBindVertexArray(0);
    }

    @Override
    public @NotNull VertexLayout layout() {
        return layout;
    }

    @Override
    public void delete() {
        if (isDeleted) { throw new VertexBufferIsDeletedException(this); }
        glDeleteBuffers(vboId);
        glDeleteVertexArrays(vaoId);
        isDeleted = true;
    }
}
