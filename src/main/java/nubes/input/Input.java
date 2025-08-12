package nubes.input;

public sealed interface Input permits ButtonInput, CloseInput, CursorInput, KeyInput, ResizeInput, ScrollInput {
}
