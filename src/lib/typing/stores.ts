import { writable } from "svelte/store";

/// whether or not the user started typing
export const in_progress = writable(false);

/// includes mistype characters
export const typed_chars = writable(0);

/// number of correctly typed characters
/// correctly retyping (after deleting) the same character doesn't increase the progress
/// so its non-decreasing
export const text_progress = writable(0);
