package nubes.renderer.texture;

import org.jetbrains.annotations.NotNull;

import java.nio.file.Path;

class UnableToLoadTextureException extends RuntimeException {
    public UnableToLoadTextureException(@NotNull Path texturePath) {
        super("Couldn't load the texture " + texturePath);
    }
}

class TextureIsDeletedException extends RuntimeException {
    public TextureIsDeletedException(@NotNull Texture texture) {
        super("The selected texture has already been deleted. Texture: " + texture);
    }
}

class UnsupportedChannelsException extends RuntimeException {
    public UnsupportedChannelsException(int channels) {
        super("A texture should have either 3 or 4 channels, input: " + channels);
    }
}