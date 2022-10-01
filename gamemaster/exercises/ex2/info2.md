Magical School:

- Generics
- Traits

An imaginary magical school has a new report card generation system written in Rust!
Currently the system only supports creating report cards where the student's grade
is represented numerically (e.g. 1.0 -> 5.5).
However, the school also issues alphabetical grades (A+ -> F-) and needs
to be able to print both types of report card!

Make the necessary code changes in the struct ReportCard and the impl block
to support alphabetical report cards. Change the Grade in the second test to "A+"
to show that your changes allow alphabetical grades.