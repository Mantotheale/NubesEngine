package nubes.renderer.buffer;

public interface IndexBuffer {
    void bind();
    void unbind();
    int count();
    void delete();
}
