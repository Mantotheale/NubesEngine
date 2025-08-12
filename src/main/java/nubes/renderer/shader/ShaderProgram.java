package nubes.renderer.shader;

import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4fc;

public interface ShaderProgram {
    void bind();
    void unbind();
    void delete();
    void setInt(@NotNull String name, int value);
    void setMat4f(@NotNull String name, @NotNull Matrix4fc value);
}
