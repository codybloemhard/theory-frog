# Theøry Frøg
## What is Theøry Frøg?
Theøry Frøg is an website that can help you with music theory!
you can query it and it will generate information for you.
### Usage
### Notes
#### Input
Type in a list of comma separated notes. Is not case sensitive.
For sharp and flats, use `#`, `b`, `♯` and `♭`.
Examples: `C#,D#,E#`, `f,a,c,e`, `bb,db,fb`.
#### Output
It will generate(if named) inversions(rotation of a chord), subchords(subsequence of a chord), chordtone wholetone scale(a way to build a scale from a tetrad),
strict chordscales(where the chordtones are on uneven scale degrees), supersequences(scales such that the notes appear in it in order and uninterrupted)
and supersets(scales such that the notes are all present in it).
#### Example
Your input: F, A, C, E, D,

Your pitchclasses: [F, A, C, E, D]

----------------------------------------
	Inversions

a(♮11♭13): [A, C, E, D, F]

d-(♮9): [D, F, A, C, E]

----------------------------------------
	SubChords

F: [F, A, C]

a: [A, C, E]

F∆: [F, A, C, E]

F(♮13): [F, A, C, D]

a(♮11): [A, C, E, D]

F∆(♮13): [F, A, C, E, D]

----------------------------------------

	Chordtone Wholetone Scale

----------------------------------------

	Strict Chordscales

F Ionian, 1ᵉ mode of Ionian: F, G, A, B♭, C, D, E

F Lydian, 4ᵉ mode of Ionian: F, G, A, B, C, D, E

F Mixolidian, 5ᵉ mode of Ionian: F, G, A, B♭, C, D, E♭

F Phrygian Dominant, 5ᵉ mode of Harmonic Minor: F, G♭, A, B♭, C, D♭, E♭

F Lydian ♯2, 6ᵉ mode of Harmonic Minor: F, G♯, A, B, C, D, E

F Harmonic Major, 1ᵉ mode of Harmonic Major: F, G, A, B♭, C, D♭, E

F Mixolydian ♭9, 5ᵉ mode of Harmonic Major: F, G♭, A, B♭, C, D, E♭

F Lydian Dominant, 4ᵉ mode of Melodic Minor: F, G, A, B, C, D, E♭

F Melodic Major, 5ᵉ mode of Melodic Minor: F, G, A, B♭, C, D♭, E♭

F Byzantine, 1ᵉ mode of Byzantine: F, G♭, A, B♭, C, D♭, E

F Lydian ♯2 ♯6, 2ᵉ mode of Byzantine: F, G♯, A, B, C, D♯, E

F Hungarian Major, 1ᵉ mode of Hungarian Major: F, G♯, A, B, C, D, E♭

F Lydian ♯6, 2ᵉ mode of Neapolitan Minor: F, G, A, B, C, D♯, E

F Ionian ♯2, 6ᵉ mode of Neapolitan Minor: F, G♯, A, B♭, C, D, E

F Lydian Dominant ♭, 4ᵉ mode of Neapolitan Major: F, G, A, B, C, D♭, E♭

F Lydian ♭6 ♭♭7*, 3ᵉ mode of Enigmatic Major: F, G, A, B, C, D♭, E♭♭

----------------------------------------

	Supersequences

----------------------------------------

	Supersets

C Ionian, 1ᵉ mode of Ionian: C, D, E, F, G, A, B

D Dorian, 2ᵉ mode of Ionian: D, E, F, G, A, B, C

G Dorian, 2ᵉ mode of Ionian: G, A, B♭, C, D, E, F

A Phrygian, 3ᵉ mode of Ionian: A, B♭, C, D, E, F, G

E Phrygian, 3ᵉ mode of Ionian: E, F, G, A, B, C, D

A♯ Lydian, 4ᵉ mode of Ionian: A♯, B♯, C♯♯, D♯♯, E♯, F♯♯, G♯♯

C Mixolidian, 5ᵉ mode of Ionian: C, D, E, F, G, A, B♭

G Mixolidian, 5ᵉ mode of Ionian: G, A, B, C, D, E, F

A Aeolian, 6ᵉ mode of Ionian: A, B, C, D, E, F, G

D Aeolian, 6ᵉ mode of Ionian: D, E, F, G, A, B♭, C

B Locrian, 7ᵉ mode of Ionian: B, C, D, E, F, G, A

E Locrian, 7ᵉ mode of Ionian: E, F, G, A, B♭, C, D

A Harmonic Minor, 1ᵉ mode of Harmonic Minor: A, B, C, D, E, F, G♯

B Locrian ♯6, 2ᵉ mode of Harmonic Minor: B, C, D, E, F, G♯, A

C Ionian ♯5, 3ᵉ mode of Harmonic Minor: C, D, E, F, G♯, A, B

D Dorian ♯4, 4ᵉ mode of Harmonic Minor: D, E, F, G♯, A, B, C

E Phrygian Dominant, 5ᵉ mode of Harmonic Minor: E, F, G♯, A, B, C, D

G♯ Superlocrian, 7ᵉ mode of Harmonic Minor: G♯, A, B, C, D, E, F

A Neapolitan Minor, 1ᵉ mode of Neapolitan Minor: A, B♭, C, D, E, F, G♯

A♯ Lydian ♯6, 2ᵉ mode of Neapolitan Minor: A♯, B♯, C♯♯, D♯♯, E♯, F♯♯♯, G♯♯

C Mixolydian Augmented, 3ᵉ mode of Neapolitan Minor: C, D, E, F, G♯, A, B♭

D Lydian Minor, 4ᵉ mode of Neapolitan Minor: D, E, F, G♯, A, B♭, C

E Locrian ♮3, 5ᵉ mode of Neapolitan Minor: E, F, G♯, A, B♭, C, D

G♯ Super Locrian Diminished ♭3, 7ᵉ mode of Neapolitan Minor: G♯, A, B♭, C, D, E, F

----------------------------------------

## Development
Theøry Frøg is a simple front-end web app, without backend.
It's made with simple Html, Css, Js, and WASM!
A Rust library that I build does the actual calculations, which is imported into this theory-frog rust crate.
The theory-frog rust lib is than compiled to WASM and used.
Theøry Frøg is made by Cody Bloemhard.
