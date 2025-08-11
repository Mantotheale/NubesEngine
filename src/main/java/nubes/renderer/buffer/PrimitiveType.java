package nubes.renderer.buffer;

import static org.lwjgl.opengl.GL11.GL_FLOAT;

public enum PrimitiveType {
    FLOAT {
        @Override
        public int size() {
            return Float.BYTES;
        }

        @Override
        public int openGLType() {
            return GL_FLOAT;
        }
    };

    public abstract int size();
    public abstract int openGLType();
}
