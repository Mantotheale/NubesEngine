package nubes.transform;

import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4f;

public record Scaling(float x, float y, float z) implements SpatialTransformation {
    public Scaling {
        if (x < 0) { throw new NegativeScaleComponentException(x); }
        if (y < 0) { throw new NegativeScaleComponentException(y); }
        if (z < 0) { throw new NegativeScaleComponentException(z); }
    }

    public Scaling(float scale) {
        this(scale, scale, scale);
    }

    @Override
    public @NotNull Matrix4f transform(@NotNull Matrix4f matrix) {
        return matrix.scale(x, y, z);
    }

    public @NotNull Scaling compose(@NotNull Scaling other) {
        return new Scaling(this.x * other.x, this.y * other.y, this.z * other.z);
    }
}
