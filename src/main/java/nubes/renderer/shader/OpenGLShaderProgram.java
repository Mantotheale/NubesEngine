package nubes.renderer.shader;

import nubes.utils.FileUtils;
import org.jetbrains.annotations.NotNull;
import org.joml.Matrix4fc;
import org.lwjgl.system.MemoryStack;

import java.nio.FloatBuffer;
import java.nio.file.Path;
import java.util.*;

import static org.lwjgl.opengl.GL20.*;

public class OpenGLShaderProgram implements ShaderProgram {
    private final int id;
    private boolean isDeleted;

    private OpenGLShaderProgram(int id) {
        this.id = id;
        this.isDeleted = false;
    }

    @Override
    public void bind() {
        if (isDeleted) { throw new ShaderProgramIsDeletedException(this); }
        glUseProgram(id);
    }

    @Override
    public void unbind() {
        if (isDeleted) { throw new ShaderProgramIsDeletedException(this); }
        glUseProgram(0);
    }

    @Override
    public void delete() {
        if (isDeleted) { throw new ShaderProgramIsDeletedException(this); }
        glDeleteProgram(id);
        isDeleted = true;
    }

    @Override
    public void setInt(@NotNull String name, int value) {
        if (isDeleted) { throw new ShaderProgramIsDeletedException(this); }
        glUniform1i(getUniformLocation(name), value);
    }

    @Override
    public void setMat4f(@NotNull String name, @NotNull Matrix4fc value) {
        if (isDeleted) { throw new ShaderProgramIsDeletedException(this); }

        try (MemoryStack stack = MemoryStack.stackPush()) {
            FloatBuffer buffer = value.get(stack.mallocFloat(16));
            glUniformMatrix4fv(getUniformLocation(name), false, buffer);
        }
    }

    private int getUniformLocation(String name) {
        int location = glGetUniformLocation(id, name);
        if (location == -1) { throw new ShaderUniformNotFoundException(this, name); }
        return location;
    }

    public static class Builder {
        private final @NotNull Map<@NotNull ShaderType, @NotNull Integer> shaders = new EnumMap<>(ShaderType.class);

        public @NotNull Builder addShader(@NotNull ShaderType type, @NotNull String content) {
            Objects.requireNonNull(content);
            int shader = glCreateShader(type.openGLType());
            glShaderSource(shader, content);
            glCompileShader(shader);

            if (glGetShaderi(shader, GL_COMPILE_STATUS) == GL_FALSE)
                throw new ShaderCompilationException(shader, glGetShaderInfoLog(shader));

            if (shaders.containsKey(type)) {
                glDeleteShader(shaders.get(type));
                shaders.remove(type);
            }

            shaders.put(type, shader);
            return this;
        }

        public @NotNull Builder addShader(@NotNull ShaderType type, @NotNull Path shaderPath) {
            return addShader(type, FileUtils.fileToString(shaderPath));
        }

        public @NotNull ShaderProgram build() {
            if (!shaders.containsKey(ShaderType.VERTEX)) { throw new VertexShaderNotProvidedException(); }
            if (!shaders.containsKey(ShaderType.FRAGMENT)) { throw new FragmentShaderNotProvidedException(); }

            int shaderProgram = glCreateProgram();

            for (ShaderType type: ShaderType.values()) {
                if (shaders.containsKey(type)) {
                    glAttachShader(shaderProgram, shaders.get(type));
                }
            }

            glLinkProgram(shaderProgram);
            if(glGetProgrami(shaderProgram, GL_LINK_STATUS) == GL_FALSE)
                throw new ShaderProgramLinkingException(shaderProgram, glGetProgramInfoLog(shaderProgram));

            for (ShaderType type: ShaderType.values()) {
                if (shaders.containsKey(type)) {
                    glDetachShader(shaderProgram, shaders.get(type));
                    glDeleteShader(shaders.get(type));
                    shaders.remove(type);
                }
            }

            return new OpenGLShaderProgram(shaderProgram);
        }
    }
}
