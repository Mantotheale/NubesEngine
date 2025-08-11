package nubes.renderer.shader;

import static org.lwjgl.opengl.GL20.GL_FRAGMENT_SHADER;
import static org.lwjgl.opengl.GL20.GL_VERTEX_SHADER;
import static org.lwjgl.opengl.GL32.GL_GEOMETRY_SHADER;

public enum ShaderType {
    VERTEX {
        @Override
        public int openGLType() {
            return GL_VERTEX_SHADER;
        }
    },
    FRAGMENT {
        @Override
        public int openGLType() {
            return GL_FRAGMENT_SHADER;
        }
    },
    GEOMETRY {
        @Override
        public int openGLType() {
            return GL_GEOMETRY_SHADER;
        }
    };

    public abstract int openGLType();
}
