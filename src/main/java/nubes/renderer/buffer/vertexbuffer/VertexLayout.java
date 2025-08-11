package nubes.renderer.buffer.vertexbuffer;

import org.jetbrains.annotations.NotNull;

import java.util.*;

public class VertexLayout implements Iterable<LayoutElement> {
    private final @NotNull List<LayoutElement> elements;
    private final int size;

    private VertexLayout(@NotNull List<LayoutElement> elements) {
        this.elements = List.copyOf(Objects.requireNonNull(elements));
        this.size = elements.stream().mapToInt(LayoutElement::size).sum();
    }

    public int size() {
        return size;
    }

    @Override
    public @NotNull Iterator<LayoutElement> iterator() {
        return elements.iterator();
    }

    public static class Builder {
        private final @NotNull List<LayoutElement> elements;
        private int index;
        private int offset;

        public Builder() {
            elements = new ArrayList<>();
            index = 0;
            offset = 0;
        }

        public @NotNull Builder addFloats(int count) {
            LayoutElement element = new LayoutElement(index, count, PrimitiveType.FLOAT, false, offset);
            elements.add(element);
            index++;
            offset += element.size();

            return this;
        }

        public @NotNull VertexLayout build() {
            if (elements.isEmpty()) { throw new EmptyLayoutException(); }

            return new VertexLayout(elements);
        }
    }
}
