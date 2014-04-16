Barsoom
=======

A Mars Terrain Simulator using voxels from MOLA MEGDRs


Build
----
Using [cargo-lite](https://github.com/cmr/cargo-lite) seeing as cargo is not yet production ready.

Data
----
Data is from [http://pds-geosciences.wustl.edu/mgs/mgs-m-mola-5-megdr-l3-v1/mgsl_300x/aareadme.txt](pds-geosciences.wustl.edu/mgs). Only low-resolution data is in git repo.

Tech
---
Currently, using a relativly naive octree representation. Working on raytracing for rendering.

Papers
------
papers which have helped me so far
1. [Roemisch](http://www.daimi.au.dk/~aquak/MasterThesisKristofRoemisch.pdf)
2. [laine](https://mediatech.aalto.fi/~samuli/publications/laine2010tr1_paper.pdf)
