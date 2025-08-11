package nubes.renderer.texture;

import org.jetbrains.annotations.NotNull;

import java.util.Optional;

public interface Texture {
    void bind(int slot);
    void unbind();
    @NotNull Optional<Integer> boundSlot();
    void delete();

    default void bind() {
        bind(0);
    }
}
