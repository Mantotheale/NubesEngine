package nubes.transform;

import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4f;
import org.joml.Matrix4fc;

import java.util.Objects;

public class Transform implements SpatialTransformation {
    private @NotNull Translation translation;
    private @NotNull Rotation rotation;
    private @NotNull Scaling scaling;
    private final @NotNull Matrix4f model;

    public Transform(@NotNull Translation translation, @NotNull Rotation rotation, @NotNull Scaling scaling) {
        this.translation = Objects.requireNonNull(translation);
        this.rotation = Objects.requireNonNull(rotation);
        this.scaling = Objects.requireNonNull(scaling);

        model = new Matrix4f();
        updateModelMatrix();
    }

    private void updateModelMatrix() {
        translation.transform(model.identity());
        rotation.transform(model);
        scaling.transform(model);
    }

    public @NotNull Matrix4fc matrix() {
        return model;
    }

    @Override
    public @NotNull Matrix4f transform(@NotNull Matrix4f matrix) {
        return matrix.mulAffine(model);
    }

    public @NotNull Translation translation() {
        return translation;
    }

    public @NotNull Rotation rotation() {
        return rotation;
    }

    public @NotNull Scaling scaling() {
        return scaling;
    }

    public void translate(@NotNull Translation translation) {
        this.translation = this.translation.compose(translation);
        updateModelMatrix();
    }

    public void rotate(@NotNull Rotation rotation) {
        this.rotation = this.rotation.compose(rotation);
        updateModelMatrix();
    }

    public void scale(@NotNull Scaling scaling) {
        this.scaling = this.scaling.compose(scaling);
        updateModelMatrix();
    }

    @Override
    public String toString() {
        return translation.toString();
    }
}
