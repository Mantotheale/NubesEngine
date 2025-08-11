package nubes.renderer.buffer;

import org.jetbrains.annotations.NotNull;

import java.util.Objects;

public record LayoutElement(
        int index,
        int primitiveCount,
        @NotNull PrimitiveType primitiveType,
        boolean isNormalized,
        int offset) {
    public LayoutElement {
        if (index < 0) { throw new InvalidLayoutElementIndexException(index); }
        if (primitiveCount <= 0) { throw new InvalidLayoutElementPrimitiveCountException(primitiveCount); }
        Objects.requireNonNull(primitiveType);
        if (offset < 0) { throw new InvalidLayoutElementOffsetException(offset); }
    }

    public int size() {
        return primitiveCount * primitiveType.size();
    }
}
