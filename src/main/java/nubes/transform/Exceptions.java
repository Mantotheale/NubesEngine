package nubes.transform;

class NegativeScaleComponentException extends RuntimeException {
    public NegativeScaleComponentException(float scale) {
        super("Scale components must be >= 0. It was " + scale);
    }
}