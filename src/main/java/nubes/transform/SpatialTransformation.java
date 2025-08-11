package nubes.transform;

import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4f;

public interface SpatialTransformation {
    @NotNull Matrix4f transform(@NotNull Matrix4f matrix);
}
