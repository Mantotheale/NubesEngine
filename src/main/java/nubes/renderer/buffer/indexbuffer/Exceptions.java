package nubes.renderer.buffer.indexbuffer;

import org.jetbrains.annotations.NotNull;

class IndexBufferIsDeletedException extends RuntimeException {
    public IndexBufferIsDeletedException(@NotNull IndexBuffer indexBuffer) {
        super("The selected index buffer has already been deleted. Buffer: " + indexBuffer);
    }
}