package nubes.renderer.buffer.indexbuffer;

public interface IndexBuffer {
    void bind();
    void unbind();
    int count();
    void delete();
}
