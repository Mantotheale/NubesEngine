package nubes.input.events;

public sealed interface Input permits ButtonInput, CloseInput, CursorInput, KeyInput, ResizeInput, ScrollInput {
}
