package nubes.renderer.shader;

import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4fc;

public interface ShaderProgram {
    void bind();
    void unbind();
    void delete();
    void setUniform(@NotNull String name, int value);
    void setUniform(@NotNull String name, @NotNull Matrix4fc value);
}
