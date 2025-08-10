package nubes.input.enums;

import org.jetbrains.annotations.NotNull;

import java.util.Collections;
import java.util.LinkedList;
import java.util.List;

import static org.lwjgl.glfw.GLFW.*;

public enum Modifier {
    CTRL,
    SHIFT,
    ALT;

    public static @NotNull List<@NotNull Modifier> fromGLFWModifier(int glfwModifier) {
        List<Modifier> mods = null;

        if ((glfwModifier & GLFW_MOD_CONTROL) > 0) {
            mods = new LinkedList<>();
            mods.add(CTRL);
        }

        if ((glfwModifier & GLFW_MOD_SHIFT) > 0) {
            if (mods == null) {
                mods = new LinkedList<>();
            }
            mods.add(SHIFT);
        }

        if ((glfwModifier & GLFW_MOD_ALT) > 0) {
            if (mods == null) {
                mods = new LinkedList<>();
            }
            mods.add(ALT);
        }

        return mods == null ? Collections.emptyList() : mods;
    }
}
