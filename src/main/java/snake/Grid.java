package snake;

import nubes.renderer.RenderComponent;
import nubes.transform.Rotation;
import nubes.transform.Scaling;
import nubes.transform.Transform;
import nubes.transform.Translation;
import org.jetbrains.annotations.NotNull;
import org.joml.Quaternionf;

public class Grid {
    private final @NotNull RenderComponent renderComponent;
    private final @NotNull Transform transform;

    public Grid() {
        renderComponent = null;
        transform = new Transform(new Translation(0, 0, 0), new Rotation(new Quaternionf()), new Scaling(1));
    }
}
