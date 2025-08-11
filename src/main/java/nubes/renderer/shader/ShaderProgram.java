package nubes.renderer.shader;

import org.jetbrains.annotations.NotNull;

public interface ShaderProgram {
    void bind();
    void unbind();
    void delete();
    void setUniform(@NotNull String name, int value);
}
