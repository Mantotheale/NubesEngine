package nubes.renderer.texture;

import org.jetbrains.annotations.NotNull;

import static org.lwjgl.opengl.GL11.GL_RGB;
import static org.lwjgl.opengl.GL11.GL_RGBA;

public enum PixelChannels {
    RGB {
        @Override
        public int openGLType() {
            return GL_RGB;
        }
    },
    RGBA {
        @Override
        public int openGLType() {
            return GL_RGBA;
        }
    };

    public abstract int openGLType();

    public static @NotNull PixelChannels fromInt(int channels) {
        return switch (channels) {
            case 3 -> RGB;
            case 4 -> RGBA;
            default -> throw new UnsupportedChannelsException(channels);
        };
    }
}