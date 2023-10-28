# Error: NO_UNSAFE_OP

This error is caused by the `op` macro and is used to indicate that a function
with an `op` attribute should marked with a function-level `unsafe` keyword.
This rule is created to enforce the developer of an operation to better control
exactly which parts of a function is indeed `unsafe`.
