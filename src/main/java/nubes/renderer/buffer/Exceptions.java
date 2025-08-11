package nubes.renderer.buffer;

import org.jetbrains.annotations.NotNull;

class InvalidLayoutElementIndexException extends RuntimeException {
    public InvalidLayoutElementIndexException(int index) {
        super("Layout elements should have indices >= 0. It was " + index);
    }
}

class InvalidLayoutElementPrimitiveCountException extends RuntimeException {
    public InvalidLayoutElementPrimitiveCountException(int primitiveCount) {
        super("Layout elements should have primitive count > 0. It was " + primitiveCount);
    }
}

class InvalidLayoutElementOffsetException extends RuntimeException {
    public InvalidLayoutElementOffsetException(int offset) {
        super("Layout elements should have offset >= 0. It was " + offset);
    }
}

class EmptyLayoutException extends RuntimeException {
    public EmptyLayoutException() {
        super("The layout you tried to build was empty");
    }
}

class VertexBufferIsDeletedException extends RuntimeException {
    public VertexBufferIsDeletedException(@NotNull VertexBuffer vertexBuffer) {
        super("The selected vertex buffer has already been deleted. Buffer: " + vertexBuffer);
    }
}