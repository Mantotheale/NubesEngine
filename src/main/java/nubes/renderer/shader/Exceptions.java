package nubes.renderer.shader;

import org.jetbrains.annotations.NotNull;

class ShaderCompilationException extends RuntimeException {
    public ShaderCompilationException(int shader, String infoLog) {
        super("Couldn't compile shader " + shader + "\nError: " + infoLog);
    }
}

class VertexShaderNotProvidedException extends RuntimeException {
    public VertexShaderNotProvidedException() {
        super("You must provide a vertex shader to create a Shader Program");
    }
}

class FragmentShaderNotProvidedException extends RuntimeException {
    public FragmentShaderNotProvidedException() {
        super("You must provide a fragment shader to create a Shader Program");
    }
}

class ShaderProgramLinkingException extends RuntimeException {
    public ShaderProgramLinkingException(int shaderProgram, String infoLog) {
        super("Couldn't link shader program:\n" + shaderProgram + "\nError: " + infoLog);
    }
}

class ShaderProgramIsDeletedException extends RuntimeException {
    public ShaderProgramIsDeletedException(@NotNull ShaderProgram shader) {
        super("The selected shader program has already been deleted. Program: " + shader);
    }
}