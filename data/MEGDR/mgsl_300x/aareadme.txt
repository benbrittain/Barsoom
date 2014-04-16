PDS_VERSION_ID                = PDS3
RECORD_TYPE                   = STREAM
OBJECT                        = TEXT
  PUBLICATION_DATE            = 2004-11-24
  NOTE                        = "The MGS MOLA Mission Experiment
    Gridded Data Record (MEGDR)"
END_OBJECT                    = TEXT
END

         The MGS MOLA Mission Experiment Gridded Data Record


1. Introduction

This document provides an overview of the Mars Global Surveyor MOLA
MEGDR Archive, including a complete listing of the contents of the
data set.  This data set contains map products derived from altimetry
data observations acquired by the Mars Global Surveyor (MGS) Mars
Orbiter Laser Altimeter (MOLA) instrument.

MOLA data products include Aggregated Experiment Data Records, or
AEDRs (raw data), Precision Experiment Data Records, or PEDRs (data
from AEDRs with precision orbit corrections applied), and Experiment
Gridded Data Records, or EGDRs (gridded products derived from PEDRs).
Two types of EGDRs are produced, Initial Experiment Gridded Data
Record (IEGDR) and Mission Experiment Gridded Data Record (MEGDR).
This archive contains MEGDR products, a result of accumulated
altimetry observations over the course of the whole MGS mission.


2. Documentation

The MEGDR data set is described in the file MEGDRDS.CAT in the CATALOG
directory.  Also in this directory are files describing the MOLA
instrument (INST.CAT), the MGS spacecraft (INSTHOST.CAT) and mission
(MISSION.CAT), personnel associated with this archive volume
(PERSON.CAT), and a bibliography of references mentioned in these
files (REF.CAT).  The files in this directory are used to generate
entries in the Planetary Data System online catalog.

The DOCUMENT directory includes the Software Interface Specification
(SIS) document for the MEGDR data set as a PDF file and as an HTML
file.


3. Data Set Description

The MEGDR products in this data set are images gridded at several
resolutions: 4, 16, 32, 64, and 128 pixels per degree. These images
provide maps of the global topography of Mars.  In addition, maps with
resolutions of 128, 256 and 512 pixels per degree are included for the
north and south polar regions of Mars.  For each map resolution, the
data set contains images of planetary radius, topography, and the
number of MOLA shots within a grid cell (referred to as counts).
Topography is computed as the planetary radius minus the areiod
raduis.  Images of the Mars areoid are provided at resolutions of 4,
16, and 32 pixels per degree.  See the DS.CAT file in the CATALOG
directory for details about the areoid model used for this data set.

The global maps are in simple cylindrical map projections using the
IAU2000 planetocentric coordinate system with east positive longitude.
The polar maps are stored in polar stereographic projections.
Information about the map projection is found in the PDS labels.
Formulas for converting between latitude-longitude and line-sample
coordinates are in the files DSMAP.CAT and DSMAP_POLAR.CAT in the
CATALOG directory.  

Simple cylindrical maps with resolutions of 4, 16, and 32 pixels per
degree are stored in a single file.  Maps with higher resolutions are
divided into tiles.  The 64 pixel per degree maps are divided into 4
separate images, with each image covering 90 degrees of latitude and
180 degrees of longitude.  The 128 pixel per degree maps cover the
latitude range of 88 degrees north to 88 degrees south and are divided
into 16 separate tiles with each tile containing 44 degrees of
latitude and 90 degrees of longitude. Each polar stereographic map is
stored in a single file.


4. File Formats

Most images within this data set are composed of 16-bit MSB-first
signed integer pixels.  The counts images for the 64 and 128 pixel per
degree resolution maps are stored as 8-bit unsigned integer pixels.
Each image has an accompanying PDS label that describes the format and
content of the image.  Each PDS label is in a separate file with the
same name as the image it describes but with the extension .LBL.

All text files in this data set are stream format files, with a
carriage return (ASCII 13) and a line feed character (ASCII 10) at the
end of the record.  This allows the files to be read by the MacOS,
Windows, LINUX, and UNIX operating systems.  System utilities are
available on the various computer types to convert this format to the
internal format if necessary.

      Macintosh - Apple File Exchange, MS-DOS to Mac option.
      Unix - Translate utility (tr -d'\15' <input_file >output_file)
           - DOS2UNIX utility (dos2unix input_file output_file)

The SIS document in the DOCUMENT directory is given as hypertext
(HTML), which can be viewed with a Web browser, and in the proprietary
Adobe Portable Document Format (PDF), which can be viewed with the
free software Acrobat Reader from Adobe Systems Incorporated.

5. Image Display

The PDS provides software for displaying PDS-labeled images on a
variety of computer platforms.  The application program for image
display is called NASAView, which has versions for SUN/SOLARIS,
WINDOWS, and LINUX platforms.  The PDS distributes NASAView through
its Web site.  Consult the PDS Web site for the status of NASAView in
terms of its capabilities and availability.  The address for the PDS
Web site is http://pds.jpl.nasa.gov/.


6. Archive Contents

Files within this data set are organized into a series of
subdirectories below the top-level directory.

Top-level Directory
-------------------

AAREADME.TXT

The file you are currently reading.

ERRATA.TXT

This file lists known errors and anomalies for this archive.  It is
occasionally updated if additional notes about the archive are needed.

VOLDESC.CAT

This text file contains a description of the volume contents as a PDS
catalog object.  It is a required file for PDS archives.

CATALOG Directory
-----------------

Files in the CATALOG directory are text files containing documentation
formatted in PDS object description language.  The files contain
information about the MGS mission and spacecraft, the MOLA instrument,
the MEGDR data set, personnel associated with the data and archive,
and references.  See the file CATINFO.TXT for details.

DOCUMENT Directory
------------------

The DOCUMENT directory contains the data set SIS document in both HTML
and Adobe Portable Document Format (PDF).  See the file DOCINFO.TXT
for additional details.

INDEX Directory
---------------

The INDEX directory contains PDS index files for this archive.  An
index file is an ASCII table with each record (or line) in the table
containing information about a single image in the archive.  See the
file INDXINFO.TXT for additional details.

Data Directories
----------------

The MEGDR data files are stored in directories based on the resolution
of the maps.  There is one directory for each map resolution within
the data set.  Data directory names for the global maps have the form
MEGxxx, where the xxx refers to the map resolution in pixels per
degree, including leading zeros.  For example, the directory MEG016
contains all the 16 pixel per degree maps.  The polar maps are stored
in a directory named POLAR.

File names for the global image maps use the format of
MEGpxxnyyyrv.IMG with the following definitions:

  p indicates the product type (A for areoid, C for counts, R for
    radius, and T for topogray)
  xx is the latitude of the upper left corner of the image
  n indicates whether the latitude is north (N) or south (S)
  yyy is the east longitude of the upper left corner of the image
  r is the map resolution using the pattern
    c =   4 pixel per degree
    e =  16 pixel per degree
    f =  32 pixel per degree
    g =  64 pixel per degree
    h = 128 pixel per degree
    i = 256 pixel per degree
    j = 512 pixel per degree
    (This convention is consistent with that used for the Mars Digital
    Image Model [MDIM] archives.)
  v is a letter indicating the product version.

The file naming scheme used for the polar maps is MEGT_p_rrr_v.IMG,
in which p is replaced by N for north pole or S for south pole, rrr
is replaced by 128, 256 or 512, and v is the product version.

The detached PDS label files use the same naming convention with .LBL
instead of the .IMG of the image file.

  

7. Whom To Contact For Information

For questions concerning this data set:

     PDS Geosciences Node
     Washington University
     Dept. of Earth and Planetary Sciences
     1 Brookings Drive
     St. Louis, MO 63130
     314-935-5493
     WWW Site: http://wwwpds.wustl.edu
     Electronic mail address: geosci@wunder.wustl.edu

     MOLA Science Team
     Code 920
     NASA / Goddard Space Flight Center
     Greenbelt, MD 20771
     WWW Site: http://ltpwww.gsfc.nasa.gov/tharsis/mola.html
     Electronic mail address: Gregory.Neumann@GSFC.NASA.GOV

8. Cognizant Persons

MOLA data were provided by David Smith, MOLA Principal Investigator,
and Greg Neumann, both of NASA/Goddard Space Flight Center.

This volume was designed and produced by Edward A. Guinness and Susan
Slavney, Planetary Data System Geosciences Node, Washington
University, St. Louis, Missouri.

9. Citation

The MOLA MEGDR data set may be cited in published literature using the
following reference.

     Smith, D., G. Neumann, R. E. Arvidson, E. A. Guinness,
     and S. Slavney, "Mars Global Surveyor Laser Altimeter Mission
     Experiment Gridded Data Record", NASA Planetary Data System,
     MGS-M-MOLA-5-MEGDR-L3-V1.0, 2003.

