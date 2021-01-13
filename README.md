# Theøry Frøg
## What is Theøry Frøg?
Theøry Frøg is an website that can help you with music theory!
you can query it and it will generate information for you.
### Usage
### Notes
#### Input
Type in a list of comma separated notes. Is not case sensitive.
For sharp and flats, use `s` and `b`.
Examples: `Cs,Ds,Es`, `f,a,c,e`, `bb,db,fb`.
#### Output
It will generate(if named) inversions(rotation of a chord), subchords(subsequence of a chord), chordtone wholetone scale(a way to build a scale from a tetrad),
strict chordscales(where the chordtones are on uneven scale degrees), supersequences(scales such that the notes appear in it in order and uninterrupted)
and supersets(scales such that the notes are all present in it).
#### Example
Your notes: [F, A, C, E, D]
----------------------------------------
Inversions:
a(♮11♭13): [A, C, E, D, F]
d-(♮9): [D, F, A, C, E]
----------------------------------------
Subchords:
F: [F, A, C]
a: [A, C, E]
F∆: [F, A, C, E]
F(♮13): [F, A, C, D]
a(♮11): [A, C, E, D]
F∆(♮13): [F, A, C, E, D]
----------------------------------------
Chordtone Wholetone Scale:
F unnamed
Strict chordscales:
F Ionian, 1ᵉ mode of Ionian
F Lydian, 4ᵉ mode of Ionian
F Lydian ♯2, 6ᵉ mode of Harmonic Minor
F Harmonic Major, 1ᵉ mode of Harmonic Major
F Byzantine, 1ᵉ mode of Byzantine
F Lydian ♯2 ♯6, 2ᵉ mode of Byzantine
F Lydian ♯6, 2ᵉ mode of Neapolitan Minor
F Ionian ♯2, 6ᵉ mode of Neapolitan Minor
Supersequences:
Supersets:
C Ionian, 1ᵉ mode of Ionian
D Dorian, 2ᵉ mode of Ionian
G Dorian, 2ᵉ mode of Ionian
A Phrygian, 3ᵉ mode of Ionian
E Phrygian, 3ᵉ mode of Ionian
A♯/B♭ Lydian, 4ᵉ mode of Ionian
C Mixolidian, 5ᵉ mode of Ionian
G Mixolidian, 5ᵉ mode of Ionian
A Aeolian, 6ᵉ mode of Ionian
D Aeolian, 6ᵉ mode of Ionian
B Locrian, 7ᵉ mode of Ionian
E Locrian, 7ᵉ mode of Ionian
A Harmonic Minor, 1ᵉ mode of Harmonic Minor
B Locrian ♯6, 2ᵉ mode of Harmonic Minor
C Ionian ♯5, 3ᵉ mode of Harmonic Minor
D Dorian ♯4, 4ᵉ mode of Harmonic Minor
E Phrygian Dominant, 5ᵉ mode of Harmonic Minor
G♯/A♭ Superlocrian, 7ᵉ mode of Harmonic Minor
A Neapolitan Minor, 1ᵉ mode of Neapolitan Minor
A♯/B♭ Lydian ♯6, 2ᵉ mode of Neapolitan Minor
C Mixolydian Augmented, 3ᵉ mode of Neapolitan Minor
D Lydian Minor, 4ᵉ mode of Neapolitan Minor
E Locrian ♮3, 5ᵉ mode of Neapolitan Minor
G♯/A♭ Super Locrian Diminished ♭3, 7ᵉ mode of Neapolitan Minor
----------------------------------------
## Development
Theøry Frøg is a simple front-end web app, without backend.
It's made with simple Html, Css, Js, and WASM!
A Rust library that I build does the actual calculations, which is imported into this theory-frog rust crate.
The theory-frog rust lib is than compiled to WASM and used.
