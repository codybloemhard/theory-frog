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
	Input

Your input: F, A, C, E, 

Numbered pitchclasses: [8, 0, 3, 7]

Named pitchclasses: [F, A, C, E]

	Inversions

a(♭6): [A, C, E, F]

	SubChords

F: [F, A, C]

a: [A, C, E]

F∆: [F, A, C, E]

	Chordtone Wholetone Scale

	Strict Chordscales

F Ionian, 1ᵉ mode of Ionian: F, G, A, B♭, C, D, E

F Lydian, 4ᵉ mode of Ionian: F, G, A, B, C, D, E

F Lydian ♯2, 6ᵉ mode of Harmonic Minor: F, G♯, A, B, C, D, E

F Harmonic Major, 1ᵉ mode of Harmonic Major: F, G, A, B♭, C, D♭, E

F Byzantine, 1ᵉ mode of Double Harmonic Major: F, G♭, A, B♭, C, D♭, E

F Lydian ♯2 ♯6, 2ᵉ mode of Double Harmonic Major: F, G♯, A, B, C, D♯, E

F Lydian ♯6, 2ᵉ mode of Neapolitan Minor: F, G, A, B, C, D♯, E

F Ionian ♯2, 6ᵉ mode of Neapolitan Minor: F, G♯, A, B♭, C, D, E

	Supersequences

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

B Locrian ♮6, 2ᵉ mode of Harmonic Minor: B, C, D, E, F, G♯, A

C Ionian ♯5, 3ᵉ mode of Harmonic Minor: C, D, E, F, G♯, A, B

D Dorian ♯4, 4ᵉ mode of Harmonic Minor: D, E, F, G♯, A, B, C

E Phrygian Dominant, 5ᵉ mode of Harmonic Minor: E, F, G♯, A, B, C, D

G♯ Superlocrian, 7ᵉ mode of Harmonic Minor: G♯, A, B, C, D, E, F

G Dorian ♭5, 2ᵉ mode of Harmonic Major: G, A, B♭, C, D♭, E, F

A Super Phrygian, 3ᵉ mode of Harmonic Major: A, B♭, C, D♭, E, F, G

A♯ Lydian Diminished, 4ᵉ mode of Harmonic Major: A♯, B♯, C♯, D♯♯, E♯, F♯♯, G♯♯

C Mixolidian ♭2, 5ᵉ mode of Harmonic Major: C, D♭, E, F, G, A, B♭

C♯ Lydian Augmented ♯2, 6ᵉ mode of Harmonic Major: C♯, D♯♯, E♯, F♯♯, G♯♯, A♯, B♯

E Locrian ♭♭7, 7ᵉ mode of Harmonic Major: E, F, G, A, B♭, C, D♭

E Byzantine, 1ᵉ mode of Double Harmonic Major: E, F, G♯, A, B, C, D♯

F♯ Lydian ♯2 ♯6, 2ᵉ mode of Double Harmonic Major: F♯, G♯♯, A♯, B♯, C♯, D♯♯, E♯

A Ultra Phrygian, 3ᵉ mode of Double Harmonic Major: A, B♭, C, D♭, E, F, G♭

G♯ Ultra Phrygian, 3ᵉ mode of Double Harmonic Major: G♯, A, B, C, D♯, E, F

A Hungarian Minor, 4ᵉ mode of Double Harmonic Major: A, B, C, D♯, E, F, G♯

A♯ Hungarian Minor, 4ᵉ mode of Double Harmonic Major: A♯, B♯, C♯, D♯♯, E♯, F♯, G♯♯

B Oriental, 5ᵉ mode of Double Harmonic Major: B, C, D♯, E, F, G♯, A

C Oriental, 5ᵉ mode of Double Harmonic Major: C, D♭, E, F, G♭, A, B♭

C Ionian ♯2 ♯5, 6ᵉ mode of Double Harmonic Major: C, D♯, E, F, G♯, A, B

C♯ Ionian ♯2 ♯5, 6ᵉ mode of Double Harmonic Major: C♯, D♯♯, E♯, F♯, G♯♯, A♯, B♯

D♯ Locrian ♭♭3 ♭♭7, 7ᵉ mode of Double Harmonic Major: D♯, E, F, G♯, A, B, C

E Locrian ♭♭3 ♭♭7, 7ᵉ mode of Double Harmonic Major: E, F, G♭, A, B♭, C, D♭

A Neapolitan Minor, 1ᵉ mode of Neapolitan Minor: A, B♭, C, D, E, F, G♯

E Neapolitan Minor, 1ᵉ mode of Neapolitan Minor: E, F, G, A, B, C, D♯

A♯ Lydian ♯6, 2ᵉ mode of Neapolitan Minor: A♯, B♯, C♯♯, D♯♯, E♯, F♯♯♯, G♯♯

C Mixolydian Augmented, 3ᵉ mode of Neapolitan Minor: C, D, E, F, G♯, A, B♭

G Mixolydian Augmented, 3ᵉ mode of Neapolitan Minor: G, A, B, C, D♯, E, F

A Lydian Minor, 4ᵉ mode of Neapolitan Minor: A, B, C, D♯, E, F, G

D Lydian Minor, 4ᵉ mode of Neapolitan Minor: D, E, F, G♯, A, B♭, C

B Locrian ♮3, 5ᵉ mode of Neapolitan Minor: B, C, D♯, E, F, G, A

E Locrian ♮3, 5ᵉ mode of Neapolitan Minor: E, F, G♯, A, B♭, C, D

C Ionian ♯2, 6ᵉ mode of Neapolitan Minor: C, D♯, E, F, G, A, B

D♯ Superlocrian ♭♭3, 7ᵉ mode of Neapolitan Minor: D♯, E, F, G, A, B, C

G♯ Superlocrian ♭♭3, 7ᵉ mode of Neapolitan Minor: G♯, A, B♭, C, D, E, F

F♯ Enigmatic Minor, 1ᵉ mode of Enigmatic Minor: F♯, G, A, B♯, C♯, D♯♯, E♯

G Lydian Augmented ♯3 ♯♯5 ♯6, 2ᵉ mode of Enigmatic Minor: G, A, B♯, C♯, D♯♯, E♯, F♯

A Hungarian Major ♯♯4 ♯5, 3ᵉ mode of Enigmatic Minor: A, B♯, C♯, D♯♯, E♯, F♯, G

C Locrian ♮3 ♭♭6 ♭♭7, 4ᵉ mode of Enigmatic Minor: C, D♭, E, F, G♭, A♭♭, B♭♭

C♯ Harmonic Major ♯2 ♭5, 5ᵉ mode of Enigmatic Minor: C♯, D♯♯, E♯, F♯, G, A, B♯

E Superlocrian ♭♭3 ♭♭4 ♭♭5, 6ᵉ mode of Enigmatic Minor: E, F, G♭, A♭♭, B♭♭, C, D♭

F Super Phrygian ♭♭3 ♮7, 7ᵉ mode of Enigmatic Minor: F, G♭, A♭♭, B♭♭, C, D♭, E

----------------------------------------

## Development
Theøry Frøg is a simple front-end web app, without backend.
It's made with simple Html, Css, Js, and WASM!
A Rust library that I build does the actual calculations, which is imported into this theory-frog rust crate (https://github.com/ocdy1001/music-theory).
The theory-frog rust lib is than compiled to WASM and used.
Theøry Frøg is made by Cody Bloemhard.
