# Lynette

This is a tool developed in [microsoft/verus-proof-synthesis](https://github.com/microsoft/verus-proof-synthesis) that is useful for doing more fine grained comparisons of Verus programs. It is used in the Verus proof synthesis work to compare original and modified versions of Verus programs to determine if changes have been made to the specification or implementation by the LLM during synthesis.

We utilize it here as part of a similar verification that operations that take place do not modify the specification or implementation of Verus programs.

**NOTE**: This is almost entirely a copy of the code in the [microsoft/verus-proof-synthesis](https://github.com/microsoft/verus-proof-synthesis) repository. We claim no original contribution for this work (which is freely licensed under the MIT license). We vendor it here for ease of use and deployment as part of the ASP libraries.
