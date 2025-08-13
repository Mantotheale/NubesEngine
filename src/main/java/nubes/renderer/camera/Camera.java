package nubes.renderer.camera;

import nubes.transform.Translation;
import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4f;
import org.joml.Matrix4fc;
import org.joml.Vector3f;
import org.joml.Vector3fc;

public class Camera {
    private final @NotNull Vector3f eye;
    private final @NotNull Vector3f center;
    private final @NotNull Vector3f up;
    private final @NotNull Matrix4f matrix;

    public Camera(@NotNull Vector3fc eye, @NotNull Vector3fc center, @NotNull Vector3fc up) {
        this.eye = new Vector3f(eye);
        this.center = new Vector3f(center);
        this.up = new Vector3f(up);

        this.matrix = new Matrix4f();
        recalculateMatrix();
    }

    public @NotNull Matrix4fc matrix() {
        return matrix;
    }

    public void move(@NotNull Translation translation) {
        this.eye.add(translation.x(), translation.y(), translation.z());
        this.center.add(translation.x(), translation.y(), translation.z());
        recalculateMatrix();
    }

    private void recalculateMatrix() {
        matrix.setLookAt(eye, center, up);
    }
}
