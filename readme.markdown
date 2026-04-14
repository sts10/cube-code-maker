# Cube Code Maker

This tool takes a list of author names and creates a Cutter-code-like "map" to encode any author name into a series of numbers. 

As an example, the family name "Smith" might be encoded as `S6187`.

The tool performs some basic letter frequency analysis to create this "map". 

This map is a Rust HashMap with 26 keys, one for each letter of the English alphabet. Each value of this HashMap is a Vector of 676 values representing all possible second and third letter combinations in a given name. These 676 values range from `01` to `99`, so that each combination will be represented in two characters. To accommodate for this, any number may be repeated any number of times across less frequently occurring letter combinations like "cgn".


