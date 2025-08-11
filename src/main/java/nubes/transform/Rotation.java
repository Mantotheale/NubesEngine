package nubes.transform;

import org.jetbrains.annotations.NotNull;
import org.joml.*;

public record Rotation(@NotNull Quaternionfc quaternion) implements SpatialTransformation {
    @Override
    public @NotNull Matrix4f transform(@NotNull Matrix4f matrix) {
        return matrix.rotate(quaternion);
    }

    public @NotNull Rotation compose(@NotNull Rotation other) {
        return new Rotation(this.quaternion.mul(other.quaternion, new Quaternionf()));
    }
}
