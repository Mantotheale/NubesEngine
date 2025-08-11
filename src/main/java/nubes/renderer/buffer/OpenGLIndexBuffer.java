package nubes.renderer.buffer;

import org.jetbrains.annotations.NotNull;

import static org.lwjgl.opengl.GL15.*;
import static org.lwjgl.opengl.GL15.GL_STATIC_DRAW;

public class OpenGLIndexBuffer implements IndexBuffer {
    private final int id;
    private final int count;
    private boolean isDeleted;

    public OpenGLIndexBuffer(int @NotNull [] indices) {
        this.id = glGenBuffers();
        this.count = indices.length;
        this.isDeleted = false;

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, id);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, indices, GL_STATIC_DRAW);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    }

    @Override
    public void bind() {
        if (isDeleted) { throw new IndexBufferIsDeletedException(this); }
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, id);
    }

    @Override
    public void unbind() {
        if (isDeleted) { throw new IndexBufferIsDeletedException(this); }
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    }

    @Override
    public int count() {
        return count;
    }

    @Override
    public void delete() {
        if (isDeleted) { throw new IndexBufferIsDeletedException(this); }
        glDeleteBuffers(id);
        isDeleted = true;
    }
}
