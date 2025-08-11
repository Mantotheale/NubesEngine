package nubes.transform;

import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4f;

public record Translation(float x, float y, float z) implements SpatialTransformation {
    @Override
    public @NotNull Matrix4f transform(@NotNull Matrix4f matrix) {
        return matrix.translate(x, y, z);
    }

    public @NotNull Translation compose(@NotNull Translation other) {
        return new Translation(this.x + other.x, this.y + other.y, this.z + other.z);
    }
}