package nubes.input.enums;

class InvalidActionCodeException extends RuntimeException {
    public InvalidActionCodeException(int action) {
        super("The specified action wasn't a valid action code. The code was " + action);
    }
}